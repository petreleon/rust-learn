// ── validate_observed_wallet_deposit_event ──

#[test]
fn valid_observed_event_passes() {
    validate_observed_wallet_deposit_event(&observed_event()).unwrap();
}

#[test]
fn rejects_invalid_observed_chain_id() {
    let mut event = observed_event();
    event.chain_id = 0;
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
    event.chain_id = -1;
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_empty_observed_transaction_hash() {
    let mut event = observed_event();
    event.transaction_hash = "   ".into();
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_negative_observed_log_index() {
    let mut event = observed_event();
    event.log_index = -1;
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_empty_observed_contract_address() {
    let mut event = observed_event();
    event.contract_address = "".into();
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_empty_observed_from_address() {
    let mut event = observed_event();
    event.from_address = "".into();
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_empty_observed_to_address() {
    let mut event = observed_event();
    event.to_address = "".into();
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_zero_or_negative_observed_amount() {
    let mut event = observed_event();
    event.amount = BigDecimal::from(0);
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
    event.amount = BigDecimal::from(-1);
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

#[test]
fn rejects_unsupported_observed_event_type() {
    let mut event = observed_event();
    event.event_type = "swap".into();
    assert!(validate_observed_wallet_deposit_event(&event).is_err());
}

// ── deposit_intent_matches_observed_event ──

#[test]
fn matching_intent_and_event() {
    assert!(deposit_intent_matches_observed_event(
        &pending_intent(),
        &observed_event()
    ));
}

#[test]
fn non_pending_intent_does_not_match() {
    let mut intent = pending_intent();
    intent.status = WALLET_DEPOSIT_STATUS_CREDITED.into();
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
    intent.status = WALLET_DEPOSIT_STATUS_AMBIGUOUS.into();
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_address_does_not_match() {
    let mut intent = pending_intent();
    intent.ethereum_address = "0xother".into();
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}
