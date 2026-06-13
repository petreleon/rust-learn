use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotification, RewardWalletCreditNotificationError,
    RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationStore,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::rewards::reward_wallet_credit_notification_mappers::{
    map_diesel_error, map_transaction_error, RewardWalletCreditNotificationTransactionError,
};
use crate::repositories::platform_repository::user_permission_platform_request;

pub struct PostgresRewardWalletCreditNotificationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardWalletCreditNotificationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardWalletCreditNotificationStore for PostgresRewardWalletCreditNotificationStore<'_> {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditNotificationError>> {
        async move {
            user_permission_platform_request(
                self.conn,
                actor_user_id,
                &Permissions::EXECUTE_REWARD_PAYOUT.to_string(),
            )
            .await
            .map_err(map_diesel_error)
        }
        .boxed()
    }

    fn notify_reward_wallet_credit(
        &mut self,
        notification: RewardWalletCreditNotification,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    > {
        async move {
            self.conn
                .transaction::<_, RewardWalletCreditNotificationTransactionError, _>(|conn| {
                    Box::pin(async move {
                        super::reward_wallet_credit_notification_transaction::notify_reward_wallet_credit(
                            conn,
                            notification,
                        )
                        .await
                    })
                })
                .await
                .map_err(map_transaction_error)
        }
        .boxed()
    }
}
