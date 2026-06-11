#[test]
fn notified_without_notification_record() {
    let cr = credit_record(None);
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_NOTIFIED), Some(&cr), None),
        "needs_notification_record"
    );
}

#[test]
fn notified_with_notification_is_reconciled() {
    assert_eq!(
        reward_reconciliation_status(
            &candidate(REWARD_STATUS_NOTIFIED),
            Some(&credit_record(Some(10))),
            None,
        ),
        "reconciled"
    );
}

#[test]
fn completed_with_all_records_is_reconciled() {
    assert_eq!(
        reward_reconciliation_status(
            &candidate(REWARD_STATUS_COMPLETED),
            Some(&credit_record(Some(10))),
            None,
        ),
        "reconciled"
    );
}

#[test]
fn amount_approved_is_pending_execution() {
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_AMOUNT_APPROVED), None, None),
        "pending_execution"
    );
}

#[test]
fn token_pending_is_pending_execution() {
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_TOKEN_PENDING), None, None),
        "pending_execution"
    );
}

#[test]
fn rejected_is_closed_without_payout() {
    for status in &[
        REWARD_STATUS_AMOUNT_REJECTED,
        REWARD_STATUS_TEACHER_REJECTED,
        REWARD_STATUS_FAILED,
    ] {
        assert_eq!(
            reward_reconciliation_status(&candidate(status), None, None),
            "closed_without_payout"
        );
    }
}
#[test]
fn pending_approval_is_pending_decision() {
    assert_eq!(
        reward_reconciliation_status(
            &candidate(REWARD_STATUS_PENDING_TEACHER_APPROVAL),
            None,
            None
        ),
        "pending_decision"
    );
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_TEACHER_APPROVED), None, None),
        "pending_decision"
    );
}

#[test]
fn any_status_with_credit_record_is_reconciled() {
    let cr = credit_record(Some(5));
    assert_eq!(
        reward_reconciliation_status(&candidate("custom_status"), Some(&cr), None),
        "reconciled"
    );
}

#[test]
fn unknown_status_without_record_is_pending() {
    assert_eq!(
        reward_reconciliation_status(&candidate("custom_status"), None, None),
        "pending"
    );
}
