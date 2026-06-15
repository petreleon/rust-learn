use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::notify_wallet_credit::{
    self, RewardWalletCreditNotificationError, RewardWalletCreditNotificationOutput,
    RewardWalletCreditNotificationUseCase,
};
use crate::infra::postgres::rewards::reward_wallet_credit_notification_store::PostgresRewardWalletCreditNotificationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresRewardWalletCreditNotificationUseCase {
    pool: DbPool,
}

impl PostgresRewardWalletCreditNotificationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardWalletCreditNotificationUseCase for PostgresRewardWalletCreditNotificationUseCase {
    fn notify_reward_wallet_credit(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardWalletCreditNotificationStore::new(&mut conn);
            notify_wallet_credit::notify_reward_wallet_credit(&mut store, candidate_id).await
        }
        .boxed()
    }

    fn notify_reward_wallet_credit_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardWalletCreditNotificationStore::new(&mut conn);
            notify_wallet_credit::notify_reward_wallet_credit_for_actor(
                &mut store,
                actor_user_id,
                candidate_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardWalletCreditNotificationUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardWalletCreditNotificationError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardWalletCreditNotificationError::Connection(error.to_string()))
    }
}
