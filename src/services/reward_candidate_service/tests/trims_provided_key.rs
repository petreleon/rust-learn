use crate::domain::rewards::candidate::event_type::{
    REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT, REWARD_EVENT_ASSESSMENT_COMPLETION,
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
use serde_json::json;

use super::super::create_reward_candidate::candidate_teacher_user_ids;
use super::super::ensure_active_reward_policy::{
    ensure_evidence_number_at_least, ensure_reward_evidence_is_eligible,
};
use super::super::ensure_exact_course_permission::normalize_idempotency_key;
use super::normalization::test_candidate;

#[test]
fn trims_provided_key() {
    assert_eq!(
        normalize_idempotency_key(Some("  my-key  ".into()), 1, 2, "test_event").unwrap(),
        "my-key"
    );
}

#[test]
fn rejects_blank_key() {
    assert!(normalize_idempotency_key(Some("   ".into()), 1, 2, "test_event").is_err());
    assert!(normalize_idempotency_key(Some("".into()), 1, 2, "test_event").is_err());
}

#[test]
fn generates_deterministic_fallback_key() {
    assert_eq!(
        normalize_idempotency_key(None, 42, 7, "course_completion").unwrap(),
        "course_completion:42:7:manual"
    );
}

#[test]
fn generated_keys_are_unique_per_combination() {
    let a = normalize_idempotency_key(None, 1, 2, "course_completion").unwrap();
    let b = normalize_idempotency_key(None, 1, 2, "assessment_completion").unwrap();
    let c = normalize_idempotency_key(None, 2, 2, "course_completion").unwrap();
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);
}

// ── ensure_evidence_number_at_least ──

#[test]
fn evidence_at_or_above_threshold_passes() {
    ensure_evidence_number_at_least(&json!({"score": 80.0}), "score", 70.0).unwrap();
    ensure_evidence_number_at_least(&json!({"score": 70.0}), "score", 70.0).unwrap();
    ensure_evidence_number_at_least(&json!({"score": 70}), "score", 70.0).unwrap();
}

#[test]
fn evidence_below_threshold_fails() {
    assert!(ensure_evidence_number_at_least(&json!({"score": 69.9}), "score", 70.0).is_err());
}

#[test]
fn missing_evidence_key_fails() {
    assert!(ensure_evidence_number_at_least(&json!({"other": 80.0}), "score", 70.0).is_err());
}

#[test]
fn non_number_evidence_fails() {
    assert!(ensure_evidence_number_at_least(&json!({"score": "high"}), "score", 70.0).is_err());
    assert!(ensure_evidence_number_at_least(&json!({"score": null}), "score", 70.0).is_err());
}

// ── ensure_reward_evidence_is_eligible ──

#[test]
fn course_completion_requires_100_percent() {
    assert!(ensure_reward_evidence_is_eligible(
        REWARD_EVENT_COURSE_COMPLETION,
        &json!({"completion_percentage": 100.0})
    )
    .is_ok());
    assert!(ensure_reward_evidence_is_eligible(
        REWARD_EVENT_COURSE_COMPLETION,
        &json!({"completion_percentage": 99.0})
    )
    .is_err());
    assert!(
        ensure_reward_evidence_is_eligible(REWARD_EVENT_COURSE_COMPLETION, &json!({})).is_err()
    );
}

#[test]
fn assessment_completion_requires_70_passing_score() {
    assert!(ensure_reward_evidence_is_eligible(
        REWARD_EVENT_ASSESSMENT_COMPLETION,
        &json!({"passing_score": 70.0})
    )
    .is_ok());
    assert!(ensure_reward_evidence_is_eligible(
        REWARD_EVENT_ASSESSMENT_COMPLETION,
        &json!({"passing_score": 69.9})
    )
    .is_err());
    assert!(
        ensure_reward_evidence_is_eligible(REWARD_EVENT_ASSESSMENT_COMPLETION, &json!({})).is_err()
    );
}
#[test]
fn manual_and_administrative_events_always_valid() {
    assert!(ensure_reward_evidence_is_eligible(REWARD_EVENT_MANUAL_COMPLETION, &json!({})).is_ok());
    assert!(ensure_reward_evidence_is_eligible(
        REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT,
        &json!({"reason": "adjustment"})
    )
    .is_ok());
}

// ── candidate_teacher_user_ids ──

#[test]
fn returns_submitter_when_no_teacher_approver() {
    let candidate = test_candidate(5, None);
    assert_eq!(candidate_teacher_user_ids(&candidate), vec![5]);
}
