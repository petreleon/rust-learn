use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::reconcile_candidate::{
    RewardReconciliation, RewardReconciliationError, RewardReconciliationOutput,
    RewardReconciliationStore,
};
use crate::infra::postgres::rewards::reward_execution_access;
use crate::infra::postgres::rewards::reward_reconciliation_mappers::{
    map_transaction_error, RewardReconciliationTransactionError,
};

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
            reward_execution_access::can_execute_reward_payout(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardReconciliationError::Database(error.to_string()))
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
