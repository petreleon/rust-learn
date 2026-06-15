use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_candidate_audit::{
    self, RewardCandidateAuditError, RewardCandidateAuditEvent, RewardCandidateAuditUseCase,
};
use crate::infra::postgres::rewards::reward_candidate_audit_store::PostgresRewardCandidateAuditStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresRewardCandidateAuditUseCase {
    pool: DbPool,
}

impl PostgresRewardCandidateAuditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardCandidateAuditUseCase for PostgresRewardCandidateAuditUseCase {
    fn list_reward_candidate_audit(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardCandidateAuditStore::new(&mut conn);
            list_candidate_audit::list_reward_candidate_audit(
                &mut store,
                actor_user_id,
                candidate_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardCandidateAuditUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardCandidateAuditError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardCandidateAuditError::Connection(error.to_string()))
    }
}
