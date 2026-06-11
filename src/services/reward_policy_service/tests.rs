use super::*;
use bigdecimal::BigDecimal;

// ── validate_amounts ──

fn bd(value: i64) -> BigDecimal {
    BigDecimal::from(value)
}

#[test]
fn valid_amounts_pass() {
    validate_amounts(&bd(100), &bd(2), Some(&bd(500)), 3600).unwrap();
}

#[test]
fn zero_token_amount_is_valid() {
    validate_amounts(&bd(0), &bd(1), None, 0).unwrap();
}

#[test]
fn negative_token_amount_fails() {
    assert!(validate_amounts(&bd(-1), &bd(1), None, 0).is_err());
}

#[test]
fn zero_multiplier_fails() {
    assert!(validate_amounts(&bd(100), &bd(0), None, 0).is_err());
}

#[test]
fn negative_multiplier_fails() {
    assert!(validate_amounts(&bd(100), &bd(-1), None, 0).is_err());
}

#[test]
fn negative_max_payout_fails() {
    assert!(validate_amounts(&bd(100), &bd(1), Some(&bd(-1)), 0).is_err());
}

#[test]
fn none_max_payout_is_valid() {
    validate_amounts(&bd(100), &bd(1), None, 0).unwrap();
}

#[test]
fn negative_cooldown_fails() {
    assert!(validate_amounts(&bd(100), &bd(1), None, -1).is_err());
}

// ── normalize_scope_type ──

#[test]
fn normalizes_valid_policy_scopes() {
    assert_eq!(
        normalize_scope_type(REWARD_POLICY_SCOPE_PLATFORM).unwrap(),
        "platform"
    );
    assert_eq!(
        normalize_scope_type(REWARD_POLICY_SCOPE_ORGANIZATION).unwrap(),
        "organization"
    );
    assert_eq!(
        normalize_scope_type(REWARD_POLICY_SCOPE_COURSE).unwrap(),
        "course"
    );
    assert_eq!(normalize_scope_type("  COURSE  ").unwrap(), "course");
}

#[test]
fn rejects_invalid_policy_scope() {
    assert!(normalize_scope_type("").is_err());
    assert!(normalize_scope_type("unknown").is_err());
}

// ── normalize_event_type ──

#[test]
fn normalizes_valid_event_types() {
    assert_eq!(
        normalize_event_type(REWARD_EVENT_COURSE_COMPLETION).unwrap(),
        "course_completion"
    );
    assert_eq!(
        normalize_event_type(REWARD_EVENT_ASSESSMENT_COMPLETION).unwrap(),
        "assessment_completion"
    );
    assert_eq!(
        normalize_event_type(REWARD_EVENT_MANUAL_COMPLETION).unwrap(),
        "manual_completion"
    );
    assert_eq!(
        normalize_event_type(REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT).unwrap(),
        "administrative_adjustment"
    );
}

#[test]
fn normalizes_event_type_whitespace_and_dashes() {
    assert_eq!(
        normalize_event_type("  course-completion  ").unwrap(),
        "course_completion"
    );
    assert_eq!(
        normalize_event_type("assessment completion").unwrap(),
        "assessment_completion"
    );
}

#[test]
fn rejects_invalid_event_type() {
    assert!(normalize_event_type("").is_err());
    assert!(normalize_event_type("unknown").is_err());
}

// ── normalize_payment_strategy ──

#[test]
fn normalizes_valid_strategies() {
    assert_eq!(
        normalize_payment_strategy(REWARD_PAYMENT_TREASURY_TRANSFER).unwrap(),
        "treasury_transfer"
    );
    assert_eq!(
        normalize_payment_strategy(REWARD_PAYMENT_MINT).unwrap(),
        "mint"
    );
    assert_eq!(
        normalize_payment_strategy(REWARD_PAYMENT_OFF_CHAIN).unwrap(),
        "off_chain"
    );
    assert_eq!(normalize_payment_strategy("  MINT  ").unwrap(), "mint");
}

#[test]
fn rejects_invalid_strategy() {
    assert!(normalize_payment_strategy("").is_err());
    assert!(normalize_payment_strategy("direct_transfer").is_err());
}
