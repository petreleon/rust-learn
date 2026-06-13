use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::decide_amount::{
    self, RewardAmountDecisionCommand, RewardAmountDecisionError, RewardAmountDecisionOutput,
    RewardAmountDecisionUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_amount_decision_store::PostgresRewardAmountDecisionStore;

#[derive(Clone)]
pub struct PostgresRewardAmountDecisionUseCase {
    pool: DbPool,
}

impl PostgresRewardAmountDecisionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardAmountDecisionUseCase for PostgresRewardAmountDecisionUseCase {
    fn decide_reward_amount(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
        command: RewardAmountDecisionCommand,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardAmountDecisionStore::new(&mut conn);
            decide_amount::decide_reward_amount(&mut store, actor_user_id, candidate_id, command)
                .await
        }
        .boxed()
    }
}

impl PostgresRewardAmountDecisionUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardAmountDecisionError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardAmountDecisionError::Connection(error.to_string()))
    }
}
