use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::record_token_confirmation::{
    self, RewardTokenConfirmationCommand, RewardTokenConfirmationError,
    RewardTokenConfirmationOutput, RewardTokenConfirmationUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_token_confirmation_store::PostgresRewardTokenConfirmationStore;

#[derive(Clone)]
pub struct PostgresRewardTokenConfirmationUseCase {
    pool: DbPool,
}

impl PostgresRewardTokenConfirmationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardTokenConfirmationUseCase for PostgresRewardTokenConfirmationUseCase {
    fn record_reward_token_confirmation(
        &self,
        candidate_id: i64,
        command: RewardTokenConfirmationCommand,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardTokenConfirmationStore::new(&mut conn);
            record_token_confirmation::record_reward_token_confirmation(
                &mut store,
                candidate_id,
                command,
            )
            .await
        }
        .boxed()
    }

    fn record_reward_token_confirmation_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
        command: RewardTokenConfirmationCommand,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardTokenConfirmationStore::new(&mut conn);
            record_token_confirmation::record_reward_token_confirmation_for_actor(
                &mut store,
                actor_user_id,
                candidate_id,
                command,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardTokenConfirmationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardTokenConfirmationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardTokenConfirmationError::Connection(error.to_string()))
    }
}
