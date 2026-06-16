use bigdecimal::BigDecimal;
use futures::executor::block_on;

use super::handler::{list_token_tax_audit, list_token_taxes, set_token_tax};
use crate::application::wallet::manage_token_tax::{
    test_support::FakeWalletTokenTaxStore, WalletTokenTaxError, WalletTokenTaxOperation,
};

#[test]
fn lists_deposit_and_retire_token_taxes() {
    let mut store = FakeWalletTokenTaxStore {
        deposit_tax: BigDecimal::from(2),
        retire_tax: BigDecimal::from(1),
        ..Default::default()
    };

    let settings = block_on(list_token_taxes(&mut store, 7)).expect("tax settings should load");

    assert!(store.checked_view_permission);
    assert_eq!(settings.deposit.operation, "deposit");
    assert_eq!(settings.deposit.tax_amount, "2");
    assert_eq!(settings.retire.operation, "retire");
    assert_eq!(settings.retire.tax_amount, "1");
    assert_eq!(
        store.loaded_operations,
        vec![
            WalletTokenTaxOperation::Deposit,
            WalletTokenTaxOperation::Retire
        ]
    );
}

#[test]
fn denies_tax_settings_without_tax_permission() {
    let mut store = FakeWalletTokenTaxStore {
        can_view_configuration: false,
        ..Default::default()
    };

    let error = block_on(list_token_taxes(&mut store, 7))
        .expect_err("settings should require tax permission");

    assert_eq!(error, WalletTokenTaxError::PermissionDenied);
    assert!(store.checked_view_permission);
    assert!(store.loaded_operations.is_empty());
}

#[test]
fn denies_set_before_validating_amount_without_permission() {
    let mut store = FakeWalletTokenTaxStore {
        can_set: false,
        ..Default::default()
    };

    let error = block_on(set_token_tax(
        &mut store,
        7,
        WalletTokenTaxOperation::Deposit,
        BigDecimal::from(-1),
    ))
    .expect_err("permission should be checked first");

    assert_eq!(error, WalletTokenTaxError::PermissionDenied);
    assert!(store.checked_permission);
    assert!(store.saved_tax.is_none());
}

#[test]
fn denies_audit_history_without_tax_permission() {
    let mut store = FakeWalletTokenTaxStore {
        can_view_configuration: false,
        ..Default::default()
    };

    let error = block_on(list_token_tax_audit(&mut store, 7))
        .expect_err("audit history should require tax permission");

    assert_eq!(error, WalletTokenTaxError::PermissionDenied);
    assert!(store.checked_view_permission);
}

#[test]
fn lists_audit_history_with_tax_permission() {
    let mut store = FakeWalletTokenTaxStore::default();
    block_on(set_token_tax(
        &mut store,
        7,
        WalletTokenTaxOperation::Deposit,
        BigDecimal::from(2),
    ))
    .expect("tax should save");

    let events = block_on(list_token_tax_audit(&mut store, 7)).expect("audit should load");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].operation, WalletTokenTaxOperation::Deposit);
    assert_eq!(events[0].previous_tax_amount, "0");
    assert_eq!(events[0].new_tax_amount, "2");
}

#[test]
fn rejects_negative_tax_after_permission_check() {
    let mut store = FakeWalletTokenTaxStore::default();

    let error = block_on(set_token_tax(
        &mut store,
        7,
        WalletTokenTaxOperation::Retire,
        BigDecimal::from(-1),
    ))
    .expect_err("negative tax should be rejected");

    assert_eq!(
        error,
        WalletTokenTaxError::InvalidInput("tax_amount cannot be negative".to_string())
    );
    assert!(store.checked_permission);
    assert!(store.saved_tax.is_none());
}

#[test]
fn saves_non_negative_tax() {
    let mut store = FakeWalletTokenTaxStore::default();

    let view = block_on(set_token_tax(
        &mut store,
        7,
        WalletTokenTaxOperation::Deposit,
        BigDecimal::from(2),
    ))
    .expect("tax should save");

    assert_eq!(view.operation, "deposit");
    assert_eq!(view.tax_amount, "2");
    assert_eq!(
        store.saved_tax,
        Some((
            7,
            WalletTokenTaxOperation::Deposit,
            BigDecimal::from(0),
            BigDecimal::from(2)
        ))
    );
}
