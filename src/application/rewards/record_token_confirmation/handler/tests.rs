use bigdecimal::BigDecimal;
use futures::future::{BoxFuture, FutureExt};

use super::{record_reward_token_confirmation, record_reward_token_confirmation_for_actor};
use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmation, RewardTokenConfirmationCommand, RewardTokenConfirmationError,
    RewardTokenConfirmationOutput, RewardTokenConfirmationStore,
};
use crate::domain::rewards::token::{RewardTokenEventType, RewardTokenTransactionType};

#[tokio::test]
async fn records_valid_confirmation_with_transaction_type() {
    let mut store = FakeStore::new(true);
    let result = record_reward_token_confirmation(&mut store, 42, valid_command())
        .await
        .unwrap();

    assert_eq!(result.candidate_id, 42);
    assert_eq!(
        store.recorded_transaction_type,
        Some(RewardTokenTransactionType::Transfer)
    );
    assert_eq!(store.recorded_actor, None);
}

#[tokio::test]
async fn actor_permission_denial_happens_before_validation() {
    let mut store = FakeStore::new(false);
    let mut command = valid_command();
    command.chain_id = 0;

    let error = record_reward_token_confirmation_for_actor(&mut store, 9, 42, command)
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardTokenConfirmationError::PermissionDenied("EXECUTE_REWARD_PAYOUT".to_string())
    );
    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 0);
}

#[tokio::test]
async fn actor_records_confirmation_after_permission() {
    let mut store = FakeStore::new(true);

    record_reward_token_confirmation_for_actor(&mut store, 9, 42, valid_command())
        .await
        .unwrap();

    assert_eq!(store.permission_checks, 1);
    assert_eq!(store.record_calls, 1);
    assert_eq!(store.recorded_actor, Some(9));
}

#[tokio::test]
async fn invalid_non_actor_scalar_does_not_call_store() {
    let mut store = FakeStore::new(true);
    let mut command = valid_command();
    command.amount = BigDecimal::from(0);

    let error = record_reward_token_confirmation(&mut store, 42, command)
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        RewardTokenConfirmationError::InvalidInput(_)
    ));
    assert_eq!(store.record_calls, 0);
}

fn valid_command() -> RewardTokenConfirmationCommand {
    RewardTokenConfirmationCommand {
        chain_id: 1,
        contract_address: "0x1234".into(),
        transaction_hash: "0xabc".into(),
        log_index: 0,
        event_type: RewardTokenEventType::Transfer,
        from_address: Some("0xfrom".into()),
        to_address: "0xto".into(),
        amount: BigDecimal::from(50),
    }
}

struct FakeStore {
    can_execute: bool,
    permission_checks: usize,
    record_calls: usize,
    recorded_actor: Option<i32>,
    recorded_transaction_type: Option<RewardTokenTransactionType>,
}

impl FakeStore {
    fn new(can_execute: bool) -> Self {
        Self {
            can_execute,
            permission_checks: 0,
            record_calls: 0,
            recorded_actor: None,
            recorded_transaction_type: None,
        }
    }
}

impl RewardTokenConfirmationStore for FakeStore {
    fn can_execute_reward_payout(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardTokenConfirmationError>> {
        async move {
            self.permission_checks += 1;
            Ok(self.can_execute)
        }
        .boxed()
    }

    fn record_reward_token_confirmation(
        &mut self,
        confirmation: RewardTokenConfirmation,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>> {
        async move {
            self.record_calls += 1;
            self.recorded_actor = confirmation.actor_user_id;
            self.recorded_transaction_type = Some(confirmation.transaction_type);
            Ok(RewardTokenConfirmationOutput {
                candidate_id: confirmation.candidate_id,
                transaction_id: 11,
                external_transaction_id: 12,
                payout_record_id: 13,
                inserted_external_transaction: true,
            })
        }
        .boxed()
    }
}
