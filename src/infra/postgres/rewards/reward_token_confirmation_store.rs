use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmation, RewardTokenConfirmationError, RewardTokenConfirmationOutput,
    RewardTokenConfirmationStore,
};
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_token_confirmation_mappers::{
    map_transaction_error, RewardTokenConfirmationTransactionError,
};

pub struct PostgresRewardTokenConfirmationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardTokenConfirmationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardTokenConfirmationStore for PostgresRewardTokenConfirmationStore<'_> {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardTokenConfirmationError>> {
        async move {
            reward_authorization_access::can_execute_reward_payout(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardTokenConfirmationError::Database(error.to_string()))
        }
        .boxed()
    }

    fn record_reward_token_confirmation(
        &mut self,
        confirmation: RewardTokenConfirmation,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>> {
        async move {
            self.conn
                .transaction::<_, RewardTokenConfirmationTransactionError, _>(|conn| {
                    Box::pin(async move {
                        super::reward_token_confirmation_transaction::record_reward_token_confirmation(
                            conn,
                            confirmation,
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
