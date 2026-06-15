use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter, RewardPolicyOutput,
};
use crate::application::rewards::ports::RewardPolicyStore;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_policy_activation::deactivate_active_policies;
use crate::infra::postgres::rewards::reward_policy_mappers::{
    map_reward_policy_error, new_reward_policy,
};
use crate::infra::postgres::rewards::reward_policy_records::{
    create_policy, list_policies, next_policy_version, RewardPolicyFilter,
};
use crate::infra::postgres::schema::{courses, organizations};

pub struct PostgresRewardPolicyStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardPolicyStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardPolicyStore for PostgresRewardPolicyStore<'_> {
    fn can_manage_reward_policies(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPolicyError>> {
        async move {
            reward_authorization_access::can_manage_reward_policy(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardPolicyError::Database(error.to_string()))
        }
        .boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardPolicyError>> {
        async move {
            organizations::table
                .find(organization_id)
                .select(organizations::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_reward_policy_error)
        }
        .boxed()
    }

    fn course_exists(&mut self, course_id: i32) -> BoxFuture<'_, Result<(), RewardPolicyError>> {
        async move {
            courses::table
                .find(course_id)
                .select(courses::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_reward_policy_error)
        }
        .boxed()
    }

    fn create_versioned_policy(
        &mut self,
        draft: RewardPolicyDraft,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>> {
        async move { create_versioned_policy(self.conn, draft).await }.boxed()
    }

    fn list_policies(
        &mut self,
        filter: RewardPolicyListFilter,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyOutput>, RewardPolicyError>> {
        async move {
            let rows = list_policies(self.conn, RewardPolicyFilter::from(filter))
                .await
                .map_err(map_reward_policy_error)?;
            rows.into_iter().map(RewardPolicyOutput::try_from).collect()
        }
        .boxed()
    }
}

async fn create_versioned_policy(
    conn: &mut AsyncPgConnection,
    draft: RewardPolicyDraft,
) -> Result<RewardPolicyOutput, RewardPolicyError> {
    let policy = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let scope_type = draft.scope_type.as_str();
                let event_type = draft.event_type.as_str();
                let version = next_policy_version(
                    conn,
                    scope_type,
                    draft.organization_id,
                    draft.course_id,
                    event_type,
                )
                .await?;

                if draft.active {
                    deactivate_active_policies(
                        conn,
                        scope_type,
                        draft.organization_id,
                        draft.course_id,
                        event_type,
                        Utc::now(),
                    )
                    .await?;
                }

                create_policy(conn, new_reward_policy(draft, version)).await
            })
        })
        .await
        .map_err(map_reward_policy_error)?;
    RewardPolicyOutput::try_from(policy)
}
