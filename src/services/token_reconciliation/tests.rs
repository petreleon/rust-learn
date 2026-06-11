use super::{validate_observed_event, ObservedTokenEvent, TokenEventKind};
use bigdecimal::BigDecimal;

fn valid_event() -> ObservedTokenEvent {
    ObservedTokenEvent {
        chain_id: 31337,
        contract_address: "0x0000000000000000000000000000000000000001".to_string(),
        transaction_hash: "0xabc".to_string(),
        log_index: 0,
        event_type: TokenEventKind::Transfer,
        from_address: Some("0x0000000000000000000000000000000000000002".to_string()),
        to_address: "0x0000000000000000000000000000000000000003".to_string(),
        amount: BigDecimal::from(100),
    }
}

#[test]
fn token_event_kind_maps_to_transaction_types() {
    assert_eq!(TokenEventKind::Mint.transaction_type(), "token_mint");
    assert_eq!(
        TokenEventKind::Transfer.transaction_type(),
        "token_transfer"
    );
    assert_eq!(TokenEventKind::Import.transaction_type(), "token_import");
}

#[test]
fn validates_required_event_fields() {
    assert!(validate_observed_event(&valid_event()).is_ok());

    let mut missing_hash = valid_event();
    missing_hash.transaction_hash = " ".to_string();
    assert_eq!(
        validate_observed_event(&missing_hash)
            .unwrap_err()
            .to_string(),
        "transaction_hash is required"
    );

    let mut zero_amount = valid_event();
    zero_amount.amount = BigDecimal::from(0);
    assert_eq!(
        validate_observed_event(&zero_amount)
            .unwrap_err()
            .to_string(),
        "amount must be positive"
    );
}
