use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::plan_payout::{
    self, RewardPayoutPlan, RewardPayoutPlanError, RewardPayoutPlanUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_payout_plan_store::PostgresRewardPayoutPlanStore;

#[derive(Clone)]
pub struct PostgresRewardPayoutPlanUseCase {
    pool: DbPool,
}

impl PostgresRewardPayoutPlanUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardPayoutPlanUseCase for PostgresRewardPayoutPlanUseCase {
    fn plan_reward_payout(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutPlan, RewardPayoutPlanError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPayoutPlanStore::new(&mut conn);
            plan_payout::plan_reward_payout(&mut store, candidate_id).await
        }
        .boxed()
    }

    fn plan_reward_payout_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutPlan, RewardPayoutPlanError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPayoutPlanStore::new(&mut conn);
            plan_payout::plan_reward_payout_for_actor(&mut store, actor_user_id, candidate_id).await
        }
        .boxed()
    }
}

impl PostgresRewardPayoutPlanUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardPayoutPlanError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardPayoutPlanError::Connection(error.to_string()))
    }
}
