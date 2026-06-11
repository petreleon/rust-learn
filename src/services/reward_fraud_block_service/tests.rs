use super::*;
use crate::models::reward_fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};

// ── normalize_scope_type ──

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
    assert!(normalize_scope_type("").is_err());
    assert!(normalize_scope_type("unknown").is_err());
}

// ── normalize_block_request ──

fn block_request(scope: &str) -> RewardFraudBlockRequest {
    RewardFraudBlockRequest {
        scope_type: scope.into(),
        teacher_user_id: None,
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "Suspicious activity".into(),
        evidence_reference: None,
        expires_at: None,
    }
}

#[test]
fn normalizes_valid_teacher_block() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    req.teacher_user_id = Some(42);
    let result = normalize_block_request(req).unwrap();
    assert_eq!(result.scope_type, "teacher");
    assert_eq!(result.teacher_user_id, Some(42));
}

#[test]
fn normalizes_valid_organization_block() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION);
    req.organization_id = Some(10);
    let result = normalize_block_request(req).unwrap();
    assert_eq!(result.scope_type, "organization");
}

#[test]
fn normalizes_valid_course_block() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_COURSE);
    req.course_id = Some(5);
    let result = normalize_block_request(req).unwrap();
    assert_eq!(result.scope_type, "course");
}

#[test]
fn normalizes_valid_policy_block() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY);
    req.reward_policy_id = Some(1);
    let result = normalize_block_request(req).unwrap();
    assert_eq!(result.scope_type, "reward_policy");
}

#[test]
fn rejects_empty_reason() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    req.teacher_user_id = Some(1);
    req.reason = "   ".into();
    assert!(normalize_block_request(req).is_err());
}

#[test]
fn rejects_zero_targets() {
    assert!(normalize_block_request(block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER)).is_err());
}

#[test]
fn rejects_multiple_targets() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    req.teacher_user_id = Some(1);
    req.course_id = Some(2);
    assert!(normalize_block_request(req).is_err());
}

#[test]
fn rejects_mismatched_scope_and_target() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    req.course_id = Some(5);
    assert!(normalize_block_request(req).is_err());
}

#[test]
fn trims_reason_and_evidence() {
    let mut req = block_request(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    req.teacher_user_id = Some(1);
    req.reason = "  reason  ".into();
    req.evidence_reference = Some("  ev  ".into());
    let result = normalize_block_request(req).unwrap();
    assert_eq!(result.reason, "reason");
    assert_eq!(result.evidence_reference, Some("ev".into()));
}

// ── required_permissions_for_scope ──

#[test]
fn teacher_scope_requires_two_permissions() {
    let perms = required_permissions_for_scope(REWARD_FRAUD_BLOCK_SCOPE_TEACHER).unwrap();
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&Permissions::BLOCK_REWARD_TEACHER));
}

#[test]
fn organization_scope_requires_two_permissions() {
    let perms = required_permissions_for_scope(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION).unwrap();
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&Permissions::BLOCK_REWARD_ORGANIZATION));
}

#[test]
fn course_and_policy_scopes_require_one_permission() {
    for scope in &[
        REWARD_FRAUD_BLOCK_SCOPE_COURSE,
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
    ] {
        let perms = required_permissions_for_scope(scope).unwrap();
        assert_eq!(perms.len(), 1);
        assert_eq!(perms[0], Permissions::MANAGE_REWARD_FRAUD_BLOCKS);
    }
}

// ── platform/org fraud notification permissions ──

#[test]
fn platform_fraud_permissions_are_correct() {
    let perms = platform_fraud_notification_permissions();
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&Permissions::VIEW_REWARD_AUDIT.to_string()));
}

#[test]
fn organization_fraud_permissions_are_correct() {
    let perms = organization_fraud_notification_permissions();
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&Permissions::VIEW_ORG_REWARD_REPORTS.to_string()));
}
