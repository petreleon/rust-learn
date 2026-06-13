#[test]
fn normalizes_teacher_rejection() {
    assert_eq!(
        normalize_teacher_decision_status("rejected").unwrap(),
        REWARD_STATUS_TEACHER_REJECTED
    );
    assert_eq!(
        normalize_teacher_decision_status("Rejected").unwrap(),
        REWARD_STATUS_TEACHER_REJECTED
    );
    assert_eq!(
        normalize_teacher_decision_status(REWARD_STATUS_TEACHER_REJECTED).unwrap(),
        REWARD_STATUS_TEACHER_REJECTED
    );
}

#[test]
fn rejects_invalid_teacher_decision() {
    assert!(normalize_teacher_decision_status("pending").is_err());
    assert!(normalize_teacher_decision_status("").is_err());
    assert!(normalize_teacher_decision_status("maybe").is_err());
}

// ── normalize_amount_decision_status ──

#[test]
fn normalizes_amount_approval_status() {
    assert_eq!(
        normalize_amount_decision_status("approved").unwrap(),
        REWARD_STATUS_AMOUNT_APPROVED
    );
    assert_eq!(
        normalize_amount_decision_status("APPROVED").unwrap(),
        REWARD_STATUS_AMOUNT_APPROVED
    );
    assert_eq!(
        normalize_amount_decision_status(REWARD_STATUS_AMOUNT_APPROVED).unwrap(),
        REWARD_STATUS_AMOUNT_APPROVED
    );
}

#[test]
fn normalizes_amount_rejection_status() {
    assert_eq!(
        normalize_amount_decision_status("rejected").unwrap(),
        REWARD_STATUS_AMOUNT_REJECTED
    );
    assert_eq!(
        normalize_amount_decision_status(REWARD_STATUS_AMOUNT_REJECTED).unwrap(),
        REWARD_STATUS_AMOUNT_REJECTED
    );
}
#[test]
fn rejects_invalid_amount_decision() {
    assert!(normalize_amount_decision_status("pending").is_err());
    assert!(normalize_amount_decision_status("").is_err());
}

// ── normalize_idempotency_key ──

#[test]
fn uses_provided_key() {
    assert_eq!(
        normalize_idempotency_key(Some("my-key".into()), 1, 2, "test_event").unwrap(),
        "my-key"
    );
}
