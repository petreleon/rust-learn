use super::*;
use crate::models::reward_candidate::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_WALLET_CREDITED,
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use serde_json::json;

fn candidate_with_status(status: &str, approved_amount: Option<BigDecimal>) -> RewardCandidate {
    RewardCandidate {
        id: 1,
        course_id: 1,
        student_user_id: 2,
        submitter_user_id: 3,
        source_scope: "course".into(),
        source_organization_id: None,
        event_type: "course_completion".into(),
        idempotency_key: "test:key".into(),
        evidence: json!({"completion_percentage": 100.0}),
        status: status.into(),
        teacher_approver_user_id: None,
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount,
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn candidate_amount_approved() -> RewardCandidate {
    candidate_with_status(REWARD_STATUS_AMOUNT_APPROVED, Some(BigDecimal::from(100)))
}

// ── ensure_candidate_ready_for_payout ──

#[test]
fn amount_approved_is_ready() {
    ensure_candidate_ready_for_payout(&candidate_amount_approved()).unwrap();
}

#[test]
fn pending_teacher_approval_is_not_ready() {
    assert!(ensure_candidate_ready_for_payout(&candidate_with_status(
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        None
    ))
    .is_err());
}

#[test]
fn teacher_approved_is_not_ready() {
    assert!(ensure_candidate_ready_for_payout(&candidate_with_status(
        REWARD_STATUS_TEACHER_APPROVED,
        None
    ))
    .is_err());
}

#[test]
fn teacher_rejected_is_not_ready() {
    assert!(ensure_candidate_ready_for_payout(&candidate_with_status(
        REWARD_STATUS_TEACHER_REJECTED,
        None
    ))
    .is_err());
}

#[test]
fn amount_rejected_is_not_ready() {
    assert!(ensure_candidate_ready_for_payout(&candidate_with_status(
        REWARD_STATUS_AMOUNT_REJECTED,
        None
    ))
    .is_err());
}

#[test]
fn completed_is_not_ready() {
    assert!(ensure_candidate_ready_for_payout(&candidate_with_status(
        REWARD_STATUS_COMPLETED,
        Some(BigDecimal::from(100))
    ))
    .is_err());
}

// ── approved_positive_amount ──

#[test]
fn returns_approved_amount_when_positive() {
    assert_eq!(
        approved_positive_amount(&candidate_with_status(
            REWARD_STATUS_AMOUNT_APPROVED,
            Some(BigDecimal::from(50))
        ))
        .unwrap(),
        BigDecimal::from(50)
    );
}
