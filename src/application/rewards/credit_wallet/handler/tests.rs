use bigdecimal::BigDecimal;
use futures::future::{BoxFuture, FutureExt};

use super::{credit_reward_wallet, credit_reward_wallet_for_actor};
use crate::application::rewards::credit_wallet::{
    RewardWalletCredit, RewardWalletCreditError, RewardWalletCreditOutput, RewardWalletCreditStore,
};

#[tokio::test]
async fn direct_wallet_credit_records_without_actor() {
    let mut store = FakeStore::new(true);
    let result = credit_reward_wallet(&mut store, 42).await.unwrap();

    assert!(result.credited);
    assert_eq!(store.permission_checks, 0);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, None);
    assert!(!store.recorded_allow_reconciliation);
}

#[tokio::test]
async fn actor_permission_denial_happens_before_credit() {
    let mut store = FakeStore::new(false);

    let error = credit_reward_wallet_for_actor(&mut store, 9, 42)
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardWalletCreditError::PermissionDenied("EXECUTE_REWARD_PAYOUT".to_string())
    );
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 0);
}

#[tokio::test]
async fn actor_wallet_credit_records_actor() {
    let mut store = FakeStore::new(true);
    let result = credit_reward_wallet_for_actor(&mut store, 9, 42)
        .await
        .unwrap();

    assert!(result.credited);
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

impl RewardWalletCreditStore for FakeStore {
    fn can_execute_reward_payout(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditError>> {
        async move {
            self.permission_checks += 1;
            Ok(self.can_execute)
        }
        .boxed()
    }

    fn credit_reward_wallet(
        &mut self,
        credit: RewardWalletCredit,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>> {
        async move {
            self.record_calls += 1;
            self.recorded_actor = credit.actor_user_id;
            self.recorded_allow_reconciliation = credit.allow_reconciliation_credit;
            Ok(RewardWalletCreditOutput {
                candidate_id: credit.candidate_id,
                wallet_id: 2,
                credit_record_id: Some(3),
                transaction_id: Some(4),
                internal_transaction_id: Some(5),
                amount: BigDecimal::from(50),
                credited: true,
            })
        }
        .boxed()
    }
}
