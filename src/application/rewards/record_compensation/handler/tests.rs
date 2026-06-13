use bigdecimal::BigDecimal;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::rewards::record_compensation::{
    record_reward_compensation, RecordRewardCompensationCommand, RewardCompensation,
    RewardCompensationError, RewardCompensationOutput, RewardCompensationRecordOutput,
    RewardCompensationStore, RewardCompensationWalletOutput,
};

#[derive(Default)]
struct FakeStore {
    can_record: bool,
    compensation: Option<RewardCompensation>,
}

impl RewardCompensationStore for FakeStore {
    fn can_record_reward_compensation(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCompensationError>> {
        ready(Ok(self.can_record)).boxed()
    }

    fn record_reward_compensation(
        &mut self,
        compensation: RewardCompensation,
    ) -> BoxFuture<'_, Result<RewardCompensationOutput, RewardCompensationError>> {
        self.compensation = Some(compensation);
        ready(Ok(output())).boxed()
    }
}

#[tokio::test]
async fn denies_before_validation_or_mutation() {
    let mut store = FakeStore::default();
    let error = record_reward_compensation(&mut store, 7, command(0, "", ""))
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardCompensationError::PermissionDenied("RECONCILE_WALLETS".to_string())
    );
    assert!(store.compensation.is_none());
}

#[tokio::test]
async fn validates_after_permission_before_mutation() {
    let mut store = FakeStore {
        can_record: true,
        compensation: None,
    };
    let error = record_reward_compensation(&mut store, 7, command(0, "manual", "key"))
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardCompensationError::InvalidInput("compensation amount cannot be zero".to_string())
    );
    assert!(store.compensation.is_none());
}

#[tokio::test]
async fn records_after_permission_and_validation() {
    let mut store = FakeStore {
        can_record: true,
        compensation: None,
    };
    let result = record_reward_compensation(&mut store, 7, command(5, "manual make-good", "key"))
        .await
        .unwrap();

    assert!(result.created);
    let compensation = store.compensation.unwrap();
    assert_eq!(compensation.actor_user_id, 7);
    assert_eq!(compensation.command.reason, "manual make-good");
}

fn command(amount: i64, reason: &str, idempotency_key: &str) -> RecordRewardCompensationCommand {
    RecordRewardCompensationCommand {
        reward_candidate_id: 11,
        amount: BigDecimal::from(amount),
        reason: reason.to_string(),
        idempotency_key: idempotency_key.to_string(),
    }
}

fn output() -> RewardCompensationOutput {
    let now = Utc::now();
    RewardCompensationOutput {
        record: RewardCompensationRecordOutput {
            id: 1,
            reward_candidate_id: 11,
            wallet_id: 22,
            transaction_id: 33,
            internal_transaction_id: 44,
            amount: BigDecimal::from(5),
            reason: "manual make-good".to_string(),
            idempotency_key: "key".to_string(),
            created_by_user_id: 7,
            created_at: now,
        },
        wallet: RewardCompensationWalletOutput {
            id: 22,
            user_id: Some(99),
            organization_id: None,
            value: BigDecimal::from(5),
        },
        created: true,
    }
}
