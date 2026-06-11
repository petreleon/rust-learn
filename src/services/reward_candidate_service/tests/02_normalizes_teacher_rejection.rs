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

// ── normalize_reward_status ──

#[test]
fn normalizes_all_valid_statuses() {
    let statuses = vec![
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        REWARD_STATUS_TEACHER_APPROVED,
        REWARD_STATUS_TEACHER_REJECTED,
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_AMOUNT_REJECTED,
        REWARD_STATUS_ADJUSTED,
        REWARD_STATUS_TOKEN_PENDING,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
        REWARD_STATUS_FAILED,
    ];
    for status in statuses {
        assert_eq!(normalize_reward_status(status).unwrap(), status);
    }
}

#[test]
fn normalizes_status_with_whitespace_and_case() {
    assert_eq!(
        normalize_reward_status("  TEACHER_APPROVED  ").unwrap(),
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert_eq!(
        normalize_reward_status("Token_Confirmed").unwrap(),
        REWARD_STATUS_TOKEN_CONFIRMED
    );
}

#[test]
fn rejects_invalid_status() {
    assert!(normalize_reward_status("").is_err());
    assert!(normalize_reward_status("not_a_status").is_err());
}

// ── normalize_idempotency_key ──

#[test]
fn uses_provided_key() {
    assert_eq!(
        normalize_idempotency_key(Some("my-key".into()), 1, 2, "test_event").unwrap(),
        "my-key"
    );
}
