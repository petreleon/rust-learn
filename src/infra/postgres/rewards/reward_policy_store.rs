use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyAuditEventOutput, RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter,
    RewardPolicyOutput, UpdateRewardPolicyActivationCommand,
};
use crate::application::rewards::ports::RewardPolicyStore;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_policy_activation_update::update_policy_activation;
use crate::infra::postgres::rewards::reward_policy_audit_records::list_policy_audit_events;
use crate::infra::postgres::rewards::reward_policy_creation::create_versioned_policy;
use crate::infra::postgres::rewards::reward_policy_mappers::map_reward_policy_error;
use crate::infra::postgres::rewards::reward_policy_records::{
    find_policy, list_policies, RewardPolicyFilter,
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

    fn update_policy_activation(
        &mut self,
        actor_user_id: i32,
        command: UpdateRewardPolicyActivationCommand,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>> {
        async move { update_policy_activation(self.conn, actor_user_id, command).await }.boxed()
    }

    fn reward_policy_exists(
        &mut self,
        policy_id: i64,
    ) -> BoxFuture<'_, Result<(), RewardPolicyError>> {
        async move {
            find_policy(self.conn, policy_id)
                .await
                .map(|_| ())
                .map_err(map_reward_policy_error)
        }
        .boxed()
    }

    fn list_policy_audit_events(
        &mut self,
        policy_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyAuditEventOutput>, RewardPolicyError>> {
        async move {
            let events = list_policy_audit_events(self.conn, policy_id)
                .await
                .map_err(map_reward_policy_error)?;
            events
                .into_iter()
                .map(RewardPolicyAuditEventOutput::try_from)
                .collect()
        }
        .boxed()
    }
}
