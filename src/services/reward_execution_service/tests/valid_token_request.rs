// ── validate_token_confirmation_request ──

fn valid_token_request() -> RewardTokenConfirmationRequest {
    RewardTokenConfirmationRequest {
        chain_id: 1,
        contract_address: "0x1234".into(),
        transaction_hash: "0xabc".into(),
        log_index: 0,
        event_type: "transfer".into(),
        from_address: Some("0xfrom".into()),
        to_address: "0xto".into(),
        amount: BigDecimal::from(50),
    }
}

#[test]
fn valid_request_passes() {
    validate_token_confirmation_request(&valid_token_request()).unwrap();
}

#[test]
fn rejects_zero_or_negative_chain_id() {
    let mut req = valid_token_request();
    req.chain_id = 0;
    assert!(validate_token_confirmation_request(&req).is_err());
    req.chain_id = -1;
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_empty_contract_address() {
    let mut req = valid_token_request();
    req.contract_address = "   ".into();
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_empty_transaction_hash() {
    let mut req = valid_token_request();
    req.transaction_hash = "".into();
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_negative_log_index() {
    let mut req = valid_token_request();
    req.log_index = -1;
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_empty_to_address() {
    let mut req = valid_token_request();
    req.to_address = "".into();
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_zero_or_negative_amount() {
    let mut req = valid_token_request();
    req.amount = BigDecimal::from(0);
    assert!(validate_token_confirmation_request(&req).is_err());
    req.amount = BigDecimal::from(-1);
    assert!(validate_token_confirmation_request(&req).is_err());
}

#[test]
fn rejects_unsupported_event_type() {
    let mut req = valid_token_request();
    req.event_type = "unknown".into();
    assert!(validate_token_confirmation_request(&req).is_err());
}

// ── transaction_type_for_event ──

#[test]
fn maps_event_to_transaction_type() {
    assert_eq!(transaction_type_for_event("mint").unwrap(), "token_mint");
    assert_eq!(
        transaction_type_for_event("transfer").unwrap(),
        "token_transfer"
    );
    assert_eq!(
        transaction_type_for_event("import").unwrap(),
        "token_import"
    );
}

#[test]
fn rejects_unknown_event() {
    assert!(transaction_type_for_event("").is_err());
    assert!(transaction_type_for_event("swap").is_err());
    assert!(transaction_type_for_event("burn").is_err());
}

// ── RewardExecutionError from diesel::Error ──

#[test]
fn diesel_not_found_maps_to_no_active_policy() {
    assert_eq!(
        RewardExecutionError::from(diesel::result::Error::NotFound),
        RewardExecutionError::NoActivePolicy
    );
}
