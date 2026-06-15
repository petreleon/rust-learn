use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::rewards::list_reward_history::{
    StudentRewardCandidateRecord, StudentRewardHistoryError, StudentRewardWalletCredit,
};
use crate::infra::postgres::rewards::reward_vocabulary::{
    parse_candidate_status, parse_reward_event_type,
};
use crate::models::reward_candidate::RewardCandidate;

pub(super) type WalletCreditRow = (
    i64,
    i32,
    i64,
    i64,
    Option<i64>,
    Option<DateTime<Utc>>,
    DateTime<Utc>,
    BigDecimal,
);

pub(super) fn candidate_record(
    candidate: RewardCandidate,
    course_title: String,
) -> Result<StudentRewardCandidateRecord, StudentRewardHistoryError> {
    let status = parse_candidate_status(&candidate.status, |message| {
        StudentRewardHistoryError::InvalidStatus(message)
    })?;
    let event_type = parse_reward_event_type(&candidate.event_type, |message| {
        StudentRewardHistoryError::Database(message)
    })?;

    Ok(StudentRewardCandidateRecord {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        course_title,
        event_type,
        status,
        approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    })
}

pub(super) fn wallet_credit(row: WalletCreditRow) -> StudentRewardWalletCredit {
    let (
        id,
        wallet_id,
        transaction_id,
        internal_id,
        notification_id,
        notified_at,
        credited_at,
        amount,
    ) = row;
    StudentRewardWalletCredit {
        reward_wallet_credit_record_id: id,
        wallet_id,
        transaction_id,
        internal_transaction_id: internal_id,
        amount: amount.to_string(),
        notification_id,
        notified_at,
        credited_at,
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::candidate_record;
    use crate::application::rewards::list_reward_history::StudentRewardHistoryError;
    use crate::domain::rewards::candidate::event_type::RewardEventType;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = candidate_record(candidate("wallet_credited"), "Rust 101".to_string())
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::WalletCredited);
        assert_eq!(mapped.event_type, RewardEventType::CourseCompletion);
        assert_eq!(mapped.approved_amount, Some("10".to_string()));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            candidate_record(candidate("not_real"), "Rust 101".to_string()).unwrap_err(),
            StudentRewardHistoryError::InvalidStatus(
                "unknown reward candidate status 'not_real'".to_string()
            )
        );
    }

    fn candidate(status: &str) -> RewardCandidate {
        RewardCandidate {
            id: 1,
            course_id: 2,
            student_user_id: 3,
            submitter_user_id: 4,
            source_scope: "course".to_string(),
            source_organization_id: None,
            event_type: "course_completion".to_string(),
            idempotency_key: "candidate:1".to_string(),
            evidence: json!({}),
            status: status.to_string(),
            teacher_approver_user_id: Some(5),
            teacher_decision_reason: Some("done".to_string()),
            teacher_decided_at: Some(Utc::now()),
            amount_reviewer_user_id: Some(6),
            approved_amount: Some(BigDecimal::from(10)),
            amount_decision_reason: Some("ok".to_string()),
            amount_decided_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub(super) fn map_reward_history_error(error: diesel::result::Error) -> StudentRewardHistoryError {
    StudentRewardHistoryError::Database(error.to_string())
}
