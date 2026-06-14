use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::reconcile_candidate::{
    self, RewardReconciliationError, RewardReconciliationOutput, RewardReconciliationUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_reconciliation_store::PostgresRewardReconciliationStore;

#[derive(Clone)]
pub struct PostgresRewardReconciliationUseCase {
    pool: DbPool,
}

impl PostgresRewardReconciliationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardReconciliationUseCase for PostgresRewardReconciliationUseCase {
    fn reconcile_reward_candidate(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardReconciliationStore::new(&mut conn);
            reconcile_candidate::reconcile_reward_candidate(&mut store, candidate_id).await
        }
        .boxed()
    }

    fn reconcile_reward_candidate_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardReconciliationStore::new(&mut conn);
            reconcile_candidate::reconcile_reward_candidate_for_actor(
                &mut store,
                actor_user_id,
                candidate_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardReconciliationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardReconciliationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardReconciliationError::Connection(error.to_string()))
    }
}
