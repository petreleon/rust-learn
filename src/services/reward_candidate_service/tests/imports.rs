use super::*;
use crate::domain::rewards::candidate::event_type::{
    REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT, REWARD_EVENT_ASSESSMENT_COMPLETION,
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED,
};
use chrono::Utc;
use serde_json::json;

fn test_candidate(
    submitter_user_id: i32,
    teacher_approver_user_id: Option<i32>,
) -> RewardCandidate {
    RewardCandidate {
        id: 1,
        course_id: 1,
        student_user_id: 2,
        submitter_user_id,
        source_scope: "course".into(),
        source_organization_id: None,
        event_type: REWARD_EVENT_COURSE_COMPLETION.into(),
        idempotency_key: "test:key".into(),
        evidence: json!({"completion_percentage": 100.0}),
        status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.into(),
        teacher_approver_user_id,
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount: None,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ── normalize_reward_event_type ──

#[test]
fn normalizes_event_type_variants() {
    assert_eq!(
        normalize_reward_event_type("assessment completion").unwrap(),
        REWARD_EVENT_ASSESSMENT_COMPLETION
    );
    assert_eq!(
        normalize_reward_event_type("Course-Completion").unwrap(),
        REWARD_EVENT_COURSE_COMPLETION
    );
    assert_eq!(
        normalize_reward_event_type("MANUAL_COMPLETION").unwrap(),
        REWARD_EVENT_MANUAL_COMPLETION
    );
    assert_eq!(
        normalize_reward_event_type("administrative_adjustment").unwrap(),
        REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT
    );
}

#[test]
fn normalizes_event_type_whitespace_and_mixed_cases() {
    assert_eq!(
        normalize_reward_event_type("  ASSESSMENT_COMPLETION  ").unwrap(),
        REWARD_EVENT_ASSESSMENT_COMPLETION
    );
    assert_eq!(
        normalize_reward_event_type("course completion").unwrap(),
        REWARD_EVENT_COURSE_COMPLETION
    );
}

#[test]
fn rejects_unknown_event_type() {
    assert!(normalize_reward_event_type("").is_err());
    assert!(normalize_reward_event_type("unknown_event").is_err());
    assert!(normalize_reward_event_type("random text").is_err());
}

// ── normalize_teacher_decision_status ──

#[test]
fn normalizes_teacher_approval() {
    assert_eq!(
        normalize_teacher_decision_status("approved").unwrap(),
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert_eq!(
        normalize_teacher_decision_status("APPROVED").unwrap(),
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert_eq!(
        normalize_teacher_decision_status("  approved  ").unwrap(),
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert_eq!(
        normalize_teacher_decision_status(REWARD_STATUS_TEACHER_APPROVED).unwrap(),
        REWARD_STATUS_TEACHER_APPROVED
    );
}
