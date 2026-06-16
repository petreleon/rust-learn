use futures::executor::block_on;

use crate::application::wallet::burn_tokens::{
    list_failed_token_burns, list_token_burn_reconciliation_queue, reconcile_token_burn,
    test_support::FakeTokenBurnStore, TokenBurnError, TokenBurnReconciliationCommand,
};

#[test]
fn reconciliation_queue_requires_reconcile_permission() {
    let mut store = FakeTokenBurnStore::default();

    let error = block_on(list_token_burn_reconciliation_queue(&mut store, 7))
        .expect_err("missing permission should fail");

    assert_eq!(error, TokenBurnError::PermissionDenied);
}

#[test]
fn failed_burn_inspection_uses_reconcile_permission() {
    let mut store = FakeTokenBurnStore {
        can_reconcile: true,
        ..Default::default()
    };

    let rows = block_on(list_failed_token_burns(&mut store, 7)).expect("failed burns");

    assert_eq!(rows[0].status, "failed");
    assert_eq!(rows[0].last_error.as_deref(), Some("indexer timeout"));
}

#[test]
fn reconcile_burn_requires_error_message_for_failure() {
    let mut store = FakeTokenBurnStore {
        can_reconcile: true,
        ..Default::default()
    };

    let error = block_on(reconcile_token_burn(
        &mut store,
        7,
        55,
        reconciliation_command(true),
    ))
    .expect_err("missing error message should fail");

    assert!(matches!(error, TokenBurnError::InvalidInput(_)));
    assert!(store.reconciled_burn.is_none());
}

#[test]
fn reconcile_burn_passes_valid_command_to_store() {
    let mut store = FakeTokenBurnStore {
        can_reconcile: true,
        ..Default::default()
    };
    let mut command = reconciliation_command(false);
    command.transaction_hash = Some("0xtx".to_string());
    command.ethereum_address = Some("0xabc".to_string());

    block_on(reconcile_token_burn(&mut store, 7, 55, command)).expect("reconcile");

    let (burn_request_id, command) = store.reconciled_burn.expect("reconciled burn");
    assert_eq!(burn_request_id, 55);
    assert_eq!(command.transaction_hash.as_deref(), Some("0xtx"));
}

fn reconciliation_command(mark_failed: bool) -> TokenBurnReconciliationCommand {
    TokenBurnReconciliationCommand {
        ethereum_address: None,
        platform_address: None,
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        mark_failed: Some(mark_failed),
        error_message: None,
    }
}
