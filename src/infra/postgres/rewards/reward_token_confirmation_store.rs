use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmation, RewardTokenConfirmationError, RewardTokenConfirmationOutput,
    RewardTokenConfirmationStore,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::rewards::reward_token_confirmation_mappers::{
    map_diesel_error, map_transaction_error, RewardTokenConfirmationTransactionError,
};
use crate::repositories::platform_repository::user_permission_platform_request;

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
