use super::*;
use bigdecimal::BigDecimal;

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
