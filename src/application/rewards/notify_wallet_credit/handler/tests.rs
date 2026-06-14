use bigdecimal::BigDecimal;
use futures::future::{BoxFuture, FutureExt};

use super::{notify_reward_wallet_credit, notify_reward_wallet_credit_for_actor};
use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotification, RewardWalletCreditNotificationError,
    RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationStore,
};

#[tokio::test]
async fn direct_notification_records_without_actor() {
    let mut store = FakeStore::new(true);
    let result = notify_reward_wallet_credit(&mut store, 42).await.unwrap();

    assert!(result.notified);
    assert_eq!(store.permission_checks, 0);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, None);
    assert!(!store.recorded_allow_reconciliation);
}

#[tokio::test]
async fn actor_permission_denial_happens_before_notification() {
    let mut store = FakeStore::new(false);

    let error = notify_reward_wallet_credit_for_actor(&mut store, 9, 42)
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardWalletCreditNotificationError::PermissionDenied("EXECUTE_REWARD_PAYOUT".to_string())
    );
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 0);
}

#[tokio::test]
async fn actor_notification_records_actor() {
    let mut store = FakeStore::new(true);
    let result = notify_reward_wallet_credit_for_actor(&mut store, 9, 42)
        .await
        .unwrap();

    assert!(result.notified);
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, Some(9));
}

struct FakeStore {
    can_execute: bool,
    permission_checks: usize,
    record_calls: usize,
    recorded_actor: Option<i32>,
    recorded_allow_reconciliation: bool,
}

impl FakeStore {
    fn new(can_execute: bool) -> Self {
        Self {
            can_execute,
            permission_checks: 0,
            record_calls: 0,
            recorded_actor: None,
            recorded_allow_reconciliation: false,
        }
    }
}

impl RewardWalletCreditNotificationStore for FakeStore {
    fn can_execute_reward_payout(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditNotificationError>> {
        async move {
            self.permission_checks += 1;
            Ok(self.can_execute)
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
            self.record_calls += 1;
            self.recorded_actor = notification.actor_user_id;
            self.recorded_allow_reconciliation = notification.allow_reconciliation_repair;
            Ok(RewardWalletCreditNotificationOutput {
                candidate_id: notification.candidate_id,
                wallet_id: 2,
                notification_id: Some(3),
                transaction_id: 4,
                amount: BigDecimal::from(50),
                notified: true,
            })
        }
        .boxed()
    }
}
