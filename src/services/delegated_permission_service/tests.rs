use super::*;
use crate::models::delegated_permission::{
    DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};

// ── normalize_permission ──

#[test]
fn normalizes_permission_string() {
    assert_eq!(
        normalize_permission("APPROVE_REWARD_AMOUNT").unwrap(),
        "APPROVE_REWARD_AMOUNT"
    );
    assert_eq!(
        normalize_permission("  EXECUTE_REWARD_PAYOUT  ").unwrap(),
        "EXECUTE_REWARD_PAYOUT"
    );
}

#[test]
fn rejects_invalid_permission() {
    assert!(normalize_permission("").is_err());
    assert!(normalize_permission("INVALID_PERMISSION").is_err());
}

// ── normalize_scope_type ──

#[test]
fn normalizes_scope_types() {
    assert_eq!(
        normalize_scope_type(DELEGATED_SCOPE_PLATFORM).unwrap(),
        "platform"
    );
    assert_eq!(
        normalize_scope_type(DELEGATED_SCOPE_ORGANIZATION).unwrap(),
        "organization"
    );
    assert_eq!(
        normalize_scope_type(DELEGATED_SCOPE_COURSE).unwrap(),
        "course"
    );
    assert_eq!(normalize_scope_type("  COURSE  ").unwrap(), "course");
    assert_eq!(
        normalize_scope_type("Organization").unwrap(),
        "organization"
    );
}

#[test]
fn rejects_invalid_scope_type() {
    assert!(normalize_scope_type("").is_err());
    assert!(normalize_scope_type("unknown").is_err());
}

// ── ensure_permission_can_be_delegated_for_reward_work ──

#[test]
fn accepts_valid_reward_permissions() {
    let valid = [
        "APPROVE_REWARD_AMOUNT",
        "EXECUTE_REWARD_PAYOUT",
        "VIEW_REWARD_AUDIT",
        "MANAGE_REWARD_FRAUD_BLOCKS",
        "BLOCK_REWARD_TEACHER",
        "BLOCK_REWARD_ORGANIZATION",
        "SUBMIT_ORG_COURSE_REWARD_EVENT",
        "VIEW_ORG_REWARD_REPORTS",
        "MANAGE_ORG_REWARD_BUDGET",
        "SUBMIT_COURSE_REWARD_EVENT",
        "CREATE_REWARDABLE_COURSE_EVENT",
        "APPROVE_STUDENT_REWARD_CANDIDATE",
        "VIEW_COURSE_REWARD_STATUS",
        "GRADE_REWARDABLE_ASSESSMENT",
        "MANAGE_COURSE_REWARD_RULES",
    ];
    for perm in valid {
        ensure_permission_can_be_delegated_for_reward_work(perm)
            .unwrap_or_else(|_| panic!("should accept {}", perm));
    }
}

#[test]
fn rejects_non_reward_permission() {
    assert!(ensure_permission_can_be_delegated_for_reward_work("MANAGE_COURSE_SETTINGS").is_err());
    assert!(ensure_permission_can_be_delegated_for_reward_work("").is_err());
}

// ── ensure_platform_permission_scope ──

#[test]
fn platform_scope_accepts_correct_permissions() {
    assert!(ensure_platform_permission_scope("APPROVE_REWARD_AMOUNT").is_ok());
    assert!(ensure_platform_permission_scope("EXECUTE_REWARD_PAYOUT").is_ok());
    assert!(ensure_platform_permission_scope("BLOCK_REWARD_TEACHER").is_ok());
}

#[test]
fn platform_scope_rejects_org_permissions() {
    assert!(ensure_platform_permission_scope("SUBMIT_ORG_COURSE_REWARD_EVENT").is_err());
    assert!(ensure_platform_permission_scope("SUBMIT_COURSE_REWARD_EVENT").is_err());
}

// ── ensure_organization_permission_scope ──

#[test]
fn org_scope_accepts_correct_permissions() {
    assert!(ensure_organization_permission_scope("SUBMIT_ORG_COURSE_REWARD_EVENT").is_ok());
    assert!(ensure_organization_permission_scope("VIEW_ORG_REWARD_REPORTS").is_ok());
    assert!(ensure_organization_permission_scope("MANAGE_ORG_REWARD_BUDGET").is_ok());
}

#[test]
fn org_scope_rejects_platform_permissions() {
    assert!(ensure_organization_permission_scope("APPROVE_REWARD_AMOUNT").is_err());
}

// ── ensure_course_permission_scope ──

#[test]
fn course_scope_accepts_correct_permissions() {
    assert!(ensure_course_permission_scope("SUBMIT_COURSE_REWARD_EVENT").is_ok());
    assert!(ensure_course_permission_scope("APPROVE_STUDENT_REWARD_CANDIDATE").is_ok());
    assert!(ensure_course_permission_scope("MANAGE_COURSE_REWARD_RULES").is_ok());
}

#[test]
fn course_scope_rejects_platform_permissions() {
    assert!(ensure_course_permission_scope("APPROVE_REWARD_AMOUNT").is_err());
}
