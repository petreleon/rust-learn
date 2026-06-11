#[test]
fn fails_when_no_approved_amount() {
    assert!(
        approved_positive_amount(&candidate_with_status(REWARD_STATUS_AMOUNT_APPROVED, None))
            .is_err()
    );
}

#[test]
fn fails_when_amount_is_zero() {
    assert!(approved_positive_amount(&candidate_with_status(
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(0))
    ))
    .is_err());
}

#[test]
fn fails_when_amount_is_negative() {
    assert!(approved_positive_amount(&candidate_with_status(
        REWARD_STATUS_AMOUNT_APPROVED,
        Some(BigDecimal::from(-1))
    ))
    .is_err());
}

// ── ensure_candidate_reconcilable ──

#[test]
fn post_amount_states_are_reconcilable() {
    for status in &[
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ] {
        ensure_candidate_reconcilable(&candidate_with_status(status, Some(BigDecimal::from(1))))
            .unwrap();
    }
}
#[test]
fn pre_amount_states_are_not_reconcilable() {
    for status in &[
        REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        REWARD_STATUS_TEACHER_APPROVED,
        REWARD_STATUS_TEACHER_REJECTED,
        REWARD_STATUS_AMOUNT_REJECTED,
    ] {
        assert!(
            ensure_candidate_reconcilable(&candidate_with_status(status, None)).is_err(),
            "status {} should not be reconcilable",
            status
        );
    }
}

// ── should_create_reconciliation_wallet_credit ──

#[test]
fn creates_credit_for_amount_approved_and_token_confirmed_and_needs_reconciliation() {
    for status in &[
        REWARD_STATUS_AMOUNT_APPROVED,
        REWARD_STATUS_TOKEN_CONFIRMED,
        REWARD_STATUS_NEEDS_RECONCILIATION,
    ] {
        assert!(
            should_create_reconciliation_wallet_credit(&candidate_with_status(
                status,
                Some(BigDecimal::from(1))
            )),
            "status {} should create reconciliation credit",
            status
        );
    }
}

#[test]
fn does_not_create_credit_for_late_states() {
    for status in &[
        REWARD_STATUS_WALLET_CREDITED,
        REWARD_STATUS_NOTIFIED,
        REWARD_STATUS_COMPLETED,
    ] {
        assert!(
            !should_create_reconciliation_wallet_credit(&candidate_with_status(
                status,
                Some(BigDecimal::from(1))
            )),
            "status {} should not create reconciliation credit",
            status
        );
    }
}
