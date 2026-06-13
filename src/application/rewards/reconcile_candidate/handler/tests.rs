use futures::future::{BoxFuture, FutureExt};

use super::{reconcile_reward_candidate, reconcile_reward_candidate_for_actor};
use crate::application::rewards::reconcile_candidate::{
    RewardReconciliation, RewardReconciliationError, RewardReconciliationOutput,
    RewardReconciliationStore,
};

#[tokio::test]
async fn direct_reconciliation_records_without_actor() {
    let mut store = FakeStore::new(true);
    let result = reconcile_reward_candidate(&mut store, 42).await.unwrap();

    assert_eq!(result.candidate_id, 42);
    assert_eq!(store.permission_checks, 0);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, None);
}

#[tokio::test]
async fn actor_permission_denial_happens_before_reconciliation() {
    let mut store = FakeStore::new(false);

    let error = reconcile_reward_candidate_for_actor(&mut store, 9, 42)
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardReconciliationError::PermissionDenied("EXECUTE_REWARD_PAYOUT".to_string())
    );
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 0);
}

#[tokio::test]
async fn actor_reconciliation_records_actor() {
    let mut store = FakeStore::new(true);
    let result = reconcile_reward_candidate_for_actor(&mut store, 9, 42)
        .await
        .unwrap();

    assert_eq!(result.candidate_id, 42);
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, Some(9));
}

struct FakeStore {
    can_execute: bool,
    permission_checks: usize,
    record_calls: usize,
    recorded_actor: Option<i32>,
}

impl FakeStore {
    fn new(can_execute: bool) -> Self {
        Self {
            can_execute,
            permission_checks: 0,
            record_calls: 0,
            recorded_actor: None,
        }
    }
}

impl RewardReconciliationStore for FakeStore {
    fn can_execute_reward_payout(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardReconciliationError>> {
        async move {
            self.permission_checks += 1;
            Ok(self.can_execute)
        }
        .boxed()
    }

    fn reconcile_reward_candidate(
        &mut self,
        reconciliation: RewardReconciliation,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>> {
        async move {
            self.record_calls += 1;
            self.recorded_actor = reconciliation.actor_user_id;
            Ok(RewardReconciliationOutput {
                candidate_id: reconciliation.candidate_id,
                wallet_credit_created: true,
                notification_created: true,
                external_transaction_link_repaired: false,
                internal_transaction_link_repaired: false,
                final_status: "notified".to_string(),
            })
        }
        .boxed()
    }
}
