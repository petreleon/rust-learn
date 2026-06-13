use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::credit_wallet::{
    self, RewardWalletCreditError, RewardWalletCreditOutput, RewardWalletCreditUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_wallet_credit_store::PostgresRewardWalletCreditStore;

#[derive(Clone)]
pub struct PostgresRewardWalletCreditUseCase {
    pool: DbPool,
}

impl PostgresRewardWalletCreditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardWalletCreditUseCase for PostgresRewardWalletCreditUseCase {
    fn credit_reward_wallet(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardWalletCreditStore::new(&mut conn);
            credit_wallet::credit_reward_wallet(&mut store, candidate_id).await
        }
        .boxed()
    }

    fn credit_reward_wallet_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardWalletCreditStore::new(&mut conn);
            credit_wallet::credit_reward_wallet_for_actor(&mut store, actor_user_id, candidate_id)
                .await
        }
        .boxed()
    }
}

impl PostgresRewardWalletCreditUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardWalletCreditError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardWalletCreditError::Connection(error.to_string()))
    }
}
