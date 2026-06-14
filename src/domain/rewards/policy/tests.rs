use super::*;

#[test]
fn exposes_stable_policy_scope_keys() {
    assert_eq!(
        RewardPolicyScope::Platform.as_str(),
        REWARD_POLICY_SCOPE_PLATFORM
    );
    assert_eq!(RewardPolicyScope::Course.as_str(), "course");
}

#[test]
fn parses_known_policy_scope() {
    assert_eq!(
        RewardPolicyScope::parse(REWARD_POLICY_SCOPE_ORGANIZATION).unwrap(),
        RewardPolicyScope::Organization
    );
}

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
    assert!(normalize_scope_type("").is_none());
    assert!(normalize_scope_type("unknown").is_none());
}

#[test]
fn exposes_stable_policy_event_keys_from_candidate_events() {
    assert_eq!(
        RewardPolicyEventType::CourseCompletion.as_str(),
        REWARD_EVENT_COURSE_COMPLETION
    );
}

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
    assert!(normalize_event_type("").is_none());
    assert!(normalize_event_type("unknown").is_none());
}

#[test]
fn exposes_stable_payment_strategy_keys() {
    assert_eq!(
        RewardPaymentStrategy::TreasuryTransfer.as_str(),
        REWARD_PAYMENT_TREASURY_TRANSFER
    );
    assert_eq!(RewardPaymentStrategy::OffChain.as_str(), "off_chain");
}

#[test]
fn parses_known_payment_strategy() {
    assert_eq!(
        RewardPaymentStrategy::parse(REWARD_PAYMENT_MINT).unwrap(),
        RewardPaymentStrategy::Mint
    );
}

#[test]
fn normalizes_valid_payment_strategies() {
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
fn rejects_invalid_payment_strategy() {
    assert!(normalize_payment_strategy("").is_none());
    assert!(normalize_payment_strategy("direct_transfer").is_none());
}
