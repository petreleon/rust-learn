use super::*;

#[test]
fn exposes_stable_scope_keys() {
    assert_eq!(
        RewardFraudBlockScope::Teacher.as_str(),
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER
    );
    assert_eq!(
        RewardFraudBlockScope::RewardPolicy.as_str(),
        "reward_policy"
    );
}

#[test]
fn parses_known_scopes() {
    assert_eq!(
        RewardFraudBlockScope::parse(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION).unwrap(),
        RewardFraudBlockScope::Organization
    );
}

#[test]
fn normalizes_valid_scopes() {
    assert_eq!(
        normalize_scope_type(REWARD_FRAUD_BLOCK_SCOPE_TEACHER).unwrap(),
        "teacher"
    );
    assert_eq!(
        normalize_scope_type(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION).unwrap(),
        "organization"
    );
    assert_eq!(
        normalize_scope_type(REWARD_FRAUD_BLOCK_SCOPE_COURSE).unwrap(),
        "course"
    );
    assert_eq!(
        normalize_scope_type(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY).unwrap(),
        "reward_policy"
    );
}

#[test]
fn rejects_invalid_scope() {
    assert!(normalize_scope_type("").is_none());
    assert!(normalize_scope_type("unknown").is_none());
}

#[test]
fn matches_exact_scope_target() {
    assert!(scope_matches_target(
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
        true,
        false,
        false,
        false,
    ));
    assert!(scope_matches_target(
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
        false,
        true,
        false,
        false,
    ));
    assert!(scope_matches_target(
        REWARD_FRAUD_BLOCK_SCOPE_COURSE,
        false,
        false,
        true,
        false,
    ));
    assert!(scope_matches_target(
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
        false,
        false,
        false,
        true,
    ));
}

#[test]
fn rejects_mismatched_scope_target() {
    assert!(!scope_matches_target(
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
        false,
        false,
        true,
        false,
    ));
}
