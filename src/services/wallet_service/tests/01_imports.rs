use super::*;
use crate::models::wallet_token_deposit_intent::{
    WalletTokenDepositIntent, WALLET_DEPOSIT_STATUS_AMBIGUOUS, WALLET_DEPOSIT_STATUS_CREDITED,
    WALLET_DEPOSIT_STATUS_PENDING,
};
use bigdecimal::BigDecimal;
use chrono::Utc;

fn pending_intent() -> WalletTokenDepositIntent {
    WalletTokenDepositIntent {
        id: 1,
        user_id: 1,
        wallet_id: 1,
        ethereum_address: "0xuser".into(),
        platform_address: "0xplatform".into(),
        amount: BigDecimal::from(100),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.to_string(),
        tax_amount: BigDecimal::from(0),
        status: WALLET_DEPOSIT_STATUS_PENDING.to_string(),
        chain_id: Some(1),
        contract_address: Some("0xcontract".into()),
        transaction_hash: Some("0xhash".into()),
        log_index: Some(0),
        event_type: None,
        external_transaction_id: None,
        transaction_id: None,
        wallet_provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK.to_string(),
        metamask_required: true,
        wallet_action: TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER.to_string(),
        last_error: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        credited_at: None,
    }
}

fn observed_event() -> ObservedWalletDepositEvent {
    ObservedWalletDepositEvent {
        chain_id: 1,
        contract_address: "0xcontract".into(),
        transaction_hash: "0xhash".into(),
        log_index: 0,
        event_type: TOKEN_TRANSFER_EVENT_TRANSFER.into(),
        from_address: "0xuser".into(),
        to_address: "0xplatform".into(),
        amount: BigDecimal::from(100),
    }
}

// ── validate_positive_amount ──

#[test]
fn positive_amount_passes() {
    validate_positive_amount(&BigDecimal::from(1), "amount").unwrap();
    validate_positive_amount(&BigDecimal::from(100), "amount").unwrap();
}

#[test]
fn zero_amount_fails() {
    assert!(validate_positive_amount(&BigDecimal::from(0), "amount").is_err());
}

#[test]
fn negative_amount_fails() {
    assert!(validate_positive_amount(&BigDecimal::from(-1), "amount").is_err());
}

// ── validate_non_negative_amount ──

#[test]
fn non_negative_amount_passes() {
    validate_non_negative_amount(&BigDecimal::from(0), "tax").unwrap();
    validate_non_negative_amount(&BigDecimal::from(10), "tax").unwrap();
}

#[test]
fn negative_amount_fails_non_negative_check() {
    assert!(validate_non_negative_amount(&BigDecimal::from(-1), "tax").is_err());
}

// ── validate_transfer_request_addresses ──

#[test]
fn valid_addresses_pass() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };
    validate_transfer_request_addresses(&req).unwrap();
}
