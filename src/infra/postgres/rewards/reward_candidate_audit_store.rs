use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::application::rewards::ports::RewardCandidateAuditStore;
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{reward_audit_events, reward_candidates};
use crate::infra::postgres::rewards::reward_candidate_audit_mappers::map_reward_candidate_audit_error;
use crate::models::reward_audit_event::RewardAuditEvent;
use crate::repositories::platform_repository::user_permission_platform_request;

pub struct PostgresRewardCandidateAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardCandidateAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardCandidateAuditStore for PostgresRewardCandidateAuditStore<'_> {
    fn can_view_reward_audit(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCandidateAuditError>> {
        async move {
            user_permission_platform_request(
                self.conn,
                actor_user_id,
                &Permissions::VIEW_REWARD_AUDIT.to_string(),
            )
            .await
            .map_err(map_reward_candidate_audit_error)
        }
        .boxed()
    }

    fn reward_candidate_exists(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<(), RewardCandidateAuditError>> {
        async move {
            reward_candidates::table
                .find(candidate_id)
                .select(reward_candidates::id)
                .first::<i64>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_reward_candidate_audit_error)
        }
        .boxed()
    }

    fn list_candidate_audit_events(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError>> {
        async move { list_candidate_audit_events(self.conn, candidate_id).await }.boxed()
    }
}

async fn list_candidate_audit_events(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError> {
    reward_audit_events::table
        .filter(reward_audit_events::reward_candidate_id.eq(candidate_id))
        .order((
            reward_audit_events::created_at.asc(),
            reward_audit_events::id.asc(),
        ))
        .select(RewardAuditEvent::as_select())
        .load::<RewardAuditEvent>(conn)
        .await
        .map(|events| {
            events
                .into_iter()
                .map(RewardCandidateAuditEvent::from)
                .collect()
        })
        .map_err(map_reward_candidate_audit_error)
}
