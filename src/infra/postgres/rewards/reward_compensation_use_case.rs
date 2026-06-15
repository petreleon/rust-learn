use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::record_compensation::{
    self, RecordRewardCompensationCommand, RewardCompensationError, RewardCompensationOutput,
    RewardCompensationUseCase,
};
use crate::infra::postgres::rewards::reward_compensation_store::PostgresRewardCompensationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresRewardCompensationUseCase {
    pool: DbPool,
}

impl PostgresRewardCompensationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardCompensationUseCase for PostgresRewardCompensationUseCase {
    fn record_reward_compensation(
        &self,
        actor_user_id: i32,
        command: RecordRewardCompensationCommand,
    ) -> BoxFuture<'_, Result<RewardCompensationOutput, RewardCompensationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardCompensationStore::new(&mut conn);
            record_compensation::record_reward_compensation(&mut store, actor_user_id, command)
                .await
        }
        .boxed()
    }
}

impl PostgresRewardCompensationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardCompensationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardCompensationError::Connection(error.to_string()))
    }
}
