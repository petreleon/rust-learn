use bigdecimal::BigDecimal;
use futures::executor::block_on;

use super::handler::retire_tokens;
use crate::application::wallet::retire_tokens::{
    test_support::FakeWalletRetirementStore, WalletRetirementCommand, WalletRetirementError,
};

#[test]
fn checks_kyc_before_request_validation() {
    let mut store = FakeWalletRetirementStore {
        user_kyc_verified: false,
        ..Default::default()
    };
    let request = request_with_amount(BigDecimal::from(0));

    let error = block_on(retire_tokens(&mut store, 7, request))
        .expect_err("unverified user should be rejected first");

    assert_eq!(error, WalletRetirementError::KycRequired);
    assert!(store.checked_kyc);
    assert!(!store.loaded_tax);
    assert!(store.retirement_draft.is_none());
}

#[test]
fn rejects_negative_amount_after_kyc() {
    let mut store = FakeWalletRetirementStore::default();
    let request = request_with_amount(BigDecimal::from(-1));

    let error =
        block_on(retire_tokens(&mut store, 7, request)).expect_err("negative amount should fail");

    assert_eq!(
        error,
        WalletRetirementError::InvalidInput("amount must be positive".to_string())
    );
    assert!(store.checked_kyc);
    assert!(!store.loaded_tax);
}

#[test]
fn creates_platform_paid_retirement_with_tax_and_platform_action() {
    let mut store = FakeWalletRetirementStore {
        platform_tax: BigDecimal::from(1),
        ..Default::default()
    };
    let request = WalletRetirementCommand {
        amount: BigDecimal::from(5),
        ethereum_address: " 0xReceiver ".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: Some(31337),
        contract_address: Some(" 0xContract ".to_string()),
        transaction_hash: Some(" RetireHash ".to_string()),
        log_index: Some(1),
        platform_address: Some(" 0xPlatform ".to_string()),
    };

    let view = block_on(retire_tokens(&mut store, 7, request)).expect("retirement should create");

    assert_eq!(view.operation, "retire");
    assert_eq!(view.amount, "5");
    assert_eq!(view.tax_amount, "1");
    assert_eq!(view.wallet_delta, "-6");
    assert_eq!(view.ethereum_address, "0xReceiver");
    assert_eq!(view.wallet_provider, "platform");
    assert!(!view.metamask_required);
    assert_eq!(view.wallet_action, "platform_transfer");
    let draft = store.retirement_draft.expect("draft should be captured");
    assert_eq!(draft.platform_address.as_deref(), Some("0xPlatform"));
    assert_eq!(draft.contract_address.as_deref(), Some("0xContract"));
    assert_eq!(draft.transaction_hash.as_deref(), Some("RetireHash"));
}

#[test]
fn creates_user_paid_retirement_without_loading_platform_tax() {
    let mut store = FakeWalletRetirementStore::default();
    let request = WalletRetirementCommand {
        amount: BigDecimal::from(5),
        ethereum_address: "0xreceiver".to_string(),
        gas_payer: "user".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };

    let view = block_on(retire_tokens(&mut store, 7, request)).expect("retirement should create");

    assert!(!store.loaded_tax);
    assert_eq!(view.tax_amount, "0");
    assert_eq!(view.wallet_provider, "metamask");
    assert!(view.metamask_required);
    assert_eq!(view.wallet_action, "metamask_presigned_transfer");
}

#[test]
fn rejects_empty_platform_address_when_provided() {
    let mut store = FakeWalletRetirementStore::default();
    let mut request = request_with_amount(BigDecimal::from(5));
    request.platform_address = Some(" ".to_string());

    let error = block_on(retire_tokens(&mut store, 7, request))
        .expect_err("empty platform address should fail");

    assert_eq!(
        error,
        WalletRetirementError::InvalidInput(
            "platform_address cannot be empty when provided".to_string()
        )
    );
}

fn request_with_amount(amount: BigDecimal) -> WalletRetirementCommand {
    WalletRetirementCommand {
        amount,
        ethereum_address: "0xreceiver".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    }
}
