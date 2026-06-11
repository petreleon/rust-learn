use super::*;
use crate::models::reward_candidate::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use serde_json::json;

fn candidate(status: &str) -> RewardCandidate {
    RewardCandidate {
        id: 1,
        course_id: 1,
        student_user_id: 2,
        submitter_user_id: 3,
        source_scope: "course".into(),
        source_organization_id: None,
        event_type: "course_completion".into(),
        idempotency_key: "key".into(),
        evidence: json!({"completion_percentage": 100.0}),
        status: status.into(),
        teacher_approver_user_id: None,
        teacher_decision_reason: None,
        teacher_decided_at: None,
        amount_reviewer_user_id: None,
        approved_amount: Some(BigDecimal::from(100)),
        amount_decision_reason: None,
        amount_decided_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn credit_record(notification_id: Option<i64>) -> RewardWalletCreditRecord {
    RewardWalletCreditRecord {
        id: 1,
        reward_candidate_id: 1,
        wallet_id: 1,
        transaction_id: 1,
        internal_transaction_id: 1,
        notification_id,
        notified_at: None,
        created_at: Utc::now(),
    }
}

fn payout_record() -> RewardPayoutRecord {
    RewardPayoutRecord {
        id: 1,
        reward_candidate_id: 1,
        transaction_id: 1,
        external_transaction_id: 1,
        created_at: Utc::now(),
    }
}

#[test]
fn needs_reconciliation_status() {
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_NEEDS_RECONCILIATION), None, None),
        "needs_reconciliation"
    );
}

#[test]
fn token_confirmed_without_payout_record() {
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_TOKEN_CONFIRMED), None, None),
        "needs_payout_record"
    );
}

#[test]
fn token_confirmed_without_credit_record() {
    assert_eq!(
        reward_reconciliation_status(
            &candidate(REWARD_STATUS_TOKEN_CONFIRMED),
            None,
            Some(&payout_record())
        ),
        "needs_wallet_credit"
    );
}

#[test]
fn wallet_credited_without_credit_record() {
    assert_eq!(
        reward_reconciliation_status(&candidate(REWARD_STATUS_WALLET_CREDITED), None, None),
        "needs_wallet_credit_record"
    );
}

#[test]
fn wallet_credited_needs_notification() {
    assert_eq!(
        reward_reconciliation_status(
            &candidate(REWARD_STATUS_WALLET_CREDITED),
            Some(&credit_record(None)),
            None,
        ),
        "needs_notification"
    );
}
