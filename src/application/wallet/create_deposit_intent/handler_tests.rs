use bigdecimal::BigDecimal;
use futures::executor::block_on;

use super::handler::create_deposit_intent;
use crate::application::wallet::create_deposit_intent::{
    test_support::FakeWalletDepositIntentStore, WalletDepositIntentError,
    WalletDepositIntentRequest,
};

#[test]
fn checks_kyc_before_request_validation() {
    let mut store = FakeWalletDepositIntentStore {
        user_kyc_verified: false,
        ..Default::default()
    };
    let request = request_with_amount(BigDecimal::from(0));

    let error = block_on(create_deposit_intent(&mut store, 7, request))
        .expect_err("unverified user should be rejected first");

    assert_eq!(error, WalletDepositIntentError::KycRequired);
    assert!(store.checked_kyc);
    assert!(!store.loaded_tax);
    assert!(store.created_draft.is_none());
}

#[test]
fn rejects_platform_paid_deposit_when_tax_exceeds_amount_before_loading_configuration() {
    let mut store = FakeWalletDepositIntentStore {
        platform_tax: BigDecimal::from(3),
        ..Default::default()
    };
    let request = request_with_amount(BigDecimal::from(2));

    let error = block_on(create_deposit_intent(&mut store, 7, request))
        .expect_err("tax larger than amount should fail");

    assert_eq!(
        error,
        WalletDepositIntentError::InvalidInput(
            "deposit amount must be greater than or equal to the platform-paid gas tax".to_string()
        )
    );
    assert!(store.loaded_tax);
    assert!(!store.loaded_configuration);
    assert!(store.created_draft.is_none());
}

#[test]
fn creates_platform_paid_deposit_intent_with_normalized_fields() {
    let mut store = FakeWalletDepositIntentStore {
        platform_tax: BigDecimal::from(2),
        configured_platform_address: "0x00000000000000000000000000000000000000bb".to_string(),
        ..Default::default()
    };
    let request = WalletDepositIntentRequest {
        amount: BigDecimal::from(20),
        ethereum_address: " 0x00000000000000000000000000000000000000AA ".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: Some(31337),
        contract_address: Some(" 0x00000000000000000000000000000000000000CC ".to_string()),
        transaction_hash: Some(" DepositHash ".to_string()),
        log_index: Some(0),
        platform_address: Some("0x00000000000000000000000000000000000000BB".to_string()),
    };

    let view =
        block_on(create_deposit_intent(&mut store, 7, request)).expect("deposit should create");

    assert_eq!(view.operation, "deposit");
    assert_eq!(view.amount, "20");
    assert_eq!(view.tax_amount, "2");
    assert_eq!(view.wallet_delta_on_confirmation, "18");
    assert_eq!(view.gas_payer, "platform");
    assert_eq!(
        view.ethereum_address,
        "0x00000000000000000000000000000000000000aa"
    );
    assert_eq!(
        view.contract_address.as_deref(),
        Some("0x00000000000000000000000000000000000000cc")
    );
    assert_eq!(view.transaction_hash.as_deref(), Some("deposithash"));
    assert_eq!(view.wallet_provider, "metamask");
    assert!(view.metamask_required);
    assert_eq!(view.wallet_action, "metamask_permit_signature");
}

#[test]
fn creates_user_paid_deposit_without_loading_platform_tax() {
    let mut store = FakeWalletDepositIntentStore {
        configured_platform_address: "0xtreasury".to_string(),
        ..Default::default()
    };
    let request = WalletDepositIntentRequest {
        amount: BigDecimal::from(5),
        ethereum_address: "0xuser".to_string(),
        gas_payer: "user".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };

    let view =
        block_on(create_deposit_intent(&mut store, 7, request)).expect("deposit should create");

    assert!(!store.loaded_tax);
    assert_eq!(view.tax_amount, "0");
    assert_eq!(view.wallet_action, "metamask_transfer");
}

#[test]
fn rejects_mismatched_platform_address() {
    let mut store = FakeWalletDepositIntentStore {
        configured_platform_address: "0xconfigured".to_string(),
        ..Default::default()
    };
    let mut request = request_with_amount(BigDecimal::from(2));
    request.platform_address = Some("0xother".to_string());

    let error = block_on(create_deposit_intent(&mut store, 7, request))
        .expect_err("platform address mismatch should fail");

    assert_eq!(
        error,
        WalletDepositIntentError::InvalidInput(
            "platform_address does not match the configured deposit receiver".to_string()
        )
    );
    assert!(store.created_draft.is_none());
}

fn request_with_amount(amount: BigDecimal) -> WalletDepositIntentRequest {
    WalletDepositIntentRequest {
        amount,
        ethereum_address: "0xuser".to_string(),
        gas_payer: "platform".to_string(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    }
}
