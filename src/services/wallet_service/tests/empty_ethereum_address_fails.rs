use bigdecimal::BigDecimal;

use super::super::support::{WalletTokenTransferRequest, TOKEN_TRANSFER_GAS_PAYER_USER};
use super::super::validate_positive_amount::{
    validate_external_transaction_fields, validate_transfer_request_addresses,
};

#[test]
fn empty_ethereum_address_fails() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "   ".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };
    assert!(validate_transfer_request_addresses(&req).is_err());
}

#[test]
fn empty_platform_address_fails() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: Some("".into()),
    };
    assert!(validate_transfer_request_addresses(&req).is_err());
}

// ── validate_external_transaction_fields ──

#[test]
fn valid_external_fields_pass() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: Some(1),
        contract_address: Some("0xabc".into()),
        transaction_hash: Some("0xdef".into()),
        log_index: Some(0),
        platform_address: None,
    };
    validate_external_transaction_fields(&req).unwrap();
}
#[test]
fn rejects_zero_or_negative_chain_id() {
    let base = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };
    let mut req = base.clone();
    req.chain_id = Some(0);
    assert!(validate_external_transaction_fields(&req).is_err());
    req.chain_id = Some(-1);
    assert!(validate_external_transaction_fields(&req).is_err());
}

#[test]
fn rejects_negative_log_index() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: None,
        log_index: Some(-1),
        platform_address: None,
    };
    assert!(validate_external_transaction_fields(&req).is_err());
}

#[test]
fn rejects_empty_contract_address_when_provided() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: Some("   ".into()),
        transaction_hash: None,
        log_index: None,
        platform_address: None,
    };
    assert!(validate_external_transaction_fields(&req).is_err());
}

#[test]
fn rejects_empty_transaction_hash_when_provided() {
    let req = WalletTokenTransferRequest {
        amount: BigDecimal::from(1),
        ethereum_address: "0x123".into(),
        gas_payer: TOKEN_TRANSFER_GAS_PAYER_USER.into(),
        chain_id: None,
        contract_address: None,
        transaction_hash: Some("".into()),
        log_index: None,
        platform_address: None,
    };
    assert!(validate_external_transaction_fields(&req).is_err());
}
