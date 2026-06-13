use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::credit_wallet::{
    RewardWalletCredit, RewardWalletCreditError, RewardWalletCreditOutput, RewardWalletCreditStore,
};
use crate::infra::postgres::rewards::reward_execution_access;
use crate::infra::postgres::rewards::reward_wallet_credit_mappers::{
    map_transaction_error, RewardWalletCreditTransactionError,
};

pub struct PostgresRewardWalletCreditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardWalletCreditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardWalletCreditStore for PostgresRewardWalletCreditStore<'_> {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditError>> {
        async move {
            reward_execution_access::can_execute_reward_payout(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardWalletCreditError::Database(error.to_string()))
        }
        .boxed()
    }

    fn credit_reward_wallet(
        &mut self,
        credit: RewardWalletCredit,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>> {
        async move {
            self.conn
                .transaction::<_, RewardWalletCreditTransactionError, _>(|conn| {
                    Box::pin(async move {
                        super::reward_wallet_credit_transaction::credit_reward_wallet(conn, credit)
                            .await
                    })
                })
                .await
                .map_err(map_transaction_error)
        }
        .boxed()
    }
}
