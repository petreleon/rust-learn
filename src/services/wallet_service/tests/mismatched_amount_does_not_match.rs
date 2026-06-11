#[test]
fn mismatched_amount_does_not_match() {
    let mut intent = pending_intent();
    intent.amount = BigDecimal::from(200);
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn platform_payer_matches_import_event() {
    let mut intent = pending_intent();
    intent.gas_payer = TOKEN_TRANSFER_GAS_PAYER_PLATFORM.into();
    let mut event = observed_event();
    event.event_type = TOKEN_TRANSFER_EVENT_IMPORT.into();
    assert!(deposit_intent_matches_observed_event(&intent, &event));
}

#[test]
fn platform_payer_does_not_match_transfer_event() {
    let mut intent = pending_intent();
    intent.gas_payer = TOKEN_TRANSFER_GAS_PAYER_PLATFORM.into();
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn unknown_gas_payer_does_not_match() {
    let mut intent = pending_intent();
    intent.gas_payer = "unknown".into();
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_chain_id_does_not_match() {
    let mut intent = pending_intent();
    intent.chain_id = Some(999);
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_contract_address_does_not_match() {
    let mut intent = pending_intent();
    intent.contract_address = Some("0xothercontract".into());
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_transaction_hash_does_not_match() {
    let mut intent = pending_intent();
    intent.transaction_hash = Some("0xotherhash".into());
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_log_index_does_not_match() {
    let mut intent = pending_intent();
    intent.log_index = Some(5);
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn mismatched_event_type_does_not_match() {
    let mut intent = pending_intent();
    intent.event_type = Some("import".into());
    assert!(!deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

#[test]
fn missing_optional_fields_match() {
    let mut intent = pending_intent();
    intent.chain_id = None;
    intent.contract_address = None;
    intent.transaction_hash = None;
    intent.log_index = None;
    intent.event_type = None;
    assert!(deposit_intent_matches_observed_event(
        &intent,
        &observed_event()
    ));
}

// ── normalize_address ──

#[test]
fn trims_and_lowercases() {
    assert_eq!(normalize_address("  0xABC  "), "0xabc");
}
