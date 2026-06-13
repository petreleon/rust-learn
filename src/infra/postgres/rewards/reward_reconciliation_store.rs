use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::reconcile_candidate::{
    RewardReconciliation, RewardReconciliationError, RewardReconciliationOutput,
    RewardReconciliationStore,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::rewards::reward_reconciliation_mappers::{
    map_diesel_error, map_transaction_error, RewardReconciliationTransactionError,
};
use crate::repositories::platform_repository::user_permission_platform_request;

pub struct PostgresRewardReconciliationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardReconciliationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardReconciliationStore for PostgresRewardReconciliationStore<'_> {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardReconciliationError>> {
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

    fn reconcile_reward_candidate(
        &mut self,
        reconciliation: RewardReconciliation,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>> {
        async move {
            self.conn
                .transaction::<_, RewardReconciliationTransactionError, _>(|conn| {
                    Box::pin(async move {
                        super::reward_reconciliation_transaction::reconcile_reward_candidate(
                            conn,
                            reconciliation,
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
