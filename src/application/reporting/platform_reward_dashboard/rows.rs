use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::application::reporting::platform_reward_dashboard::{
    classify_reward_reconciliation_mismatch, RewardCandidateDashboardRowOutput,
    RewardExecutionFailureRowOutput, RewardReconciliationMismatchFacts,
    RewardReconciliationMismatchRowOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RewardCandidateDashboardRowFact {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<BigDecimal>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) fn reward_candidate_dashboard_row(
    fact: RewardCandidateDashboardRowFact,
) -> RewardCandidateDashboardRowOutput {
    RewardCandidateDashboardRowOutput {
        reward_candidate_id: fact.reward_candidate_id,
        course_id: fact.course_id,
        student_user_id: fact.student_user_id,
        submitter_user_id: fact.submitter_user_id,
        source_organization_id: fact.source_organization_id,
        event_type: fact.event_type,
        status: fact.status,
        approved_amount: fact.approved_amount.as_ref().map(ToString::to_string),
        updated_at: fact.updated_at,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RewardExecutionFailureRowFact {
    pub reward_execution_job_id: i64,
    pub reward_candidate_id: i64,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) fn reward_execution_failure_row(
    fact: RewardExecutionFailureRowFact,
) -> RewardExecutionFailureRowOutput {
    RewardExecutionFailureRowOutput {
        reward_execution_job_id: fact.reward_execution_job_id,
        reward_candidate_id: fact.reward_candidate_id,
        status: fact.status,
        attempts: fact.attempts,
        last_error: fact.last_error,
        updated_at: fact.updated_at,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RewardReconciliationMismatchRowFact {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub status: RewardCandidateStatus,
    pub approved_amount: Option<BigDecimal>,
    pub updated_at: DateTime<Utc>,
    pub has_payout_record: bool,
    pub has_wallet_credit_record: bool,
    pub has_notification_record: bool,
}

pub(crate) fn reward_reconciliation_mismatch_row(
    fact: RewardReconciliationMismatchRowFact,
) -> Option<RewardReconciliationMismatchRowOutput> {
    let mismatch_type =
        classify_reward_reconciliation_mismatch(RewardReconciliationMismatchFacts {
            status: fact.status,
            has_payout_record: fact.has_payout_record,
            has_wallet_credit_record: fact.has_wallet_credit_record,
            has_notification_record: fact.has_notification_record,
        })?;

    Some(RewardReconciliationMismatchRowOutput {
        reward_candidate_id: fact.reward_candidate_id,
        course_id: fact.course_id,
        student_user_id: fact.student_user_id,
        status: fact.status.as_str().to_string(),
        mismatch_type: mismatch_type.as_str().to_string(),
        approved_amount: fact.approved_amount.as_ref().map(ToString::to_string),
        updated_at: fact.updated_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_pending_amount_dashboard_row_amount() {
        let now = Utc::now();

        let row = reward_candidate_dashboard_row(RewardCandidateDashboardRowFact {
            reward_candidate_id: 7,
            course_id: 8,
            student_user_id: 9,
            submitter_user_id: 10,
            source_organization_id: Some(11),
            event_type: "course_completion".to_string(),
            status: "teacher_approved".to_string(),
            approved_amount: Some(BigDecimal::from(25)),
            updated_at: now,
        });

        assert_eq!(row.reward_candidate_id, 7);
        assert_eq!(row.approved_amount, Some("25".to_string()));
    }

    #[test]
    fn builds_execution_failure_dashboard_row() {
        let now = Utc::now();

        let row = reward_execution_failure_row(RewardExecutionFailureRowFact {
            reward_execution_job_id: 1,
            reward_candidate_id: 2,
            status: "failed".to_string(),
            attempts: 3,
            last_error: Some("boom".to_string()),
            updated_at: now,
        });

        assert_eq!(row.reward_execution_job_id, 1);
        assert_eq!(row.last_error, Some("boom".to_string()));
    }

    #[test]
    fn builds_reconciliation_mismatch_dashboard_row() {
        let now = Utc::now();

        let row = reward_reconciliation_mismatch_row(RewardReconciliationMismatchRowFact {
            reward_candidate_id: 1,
            course_id: 2,
            student_user_id: 3,
            status: RewardCandidateStatus::TokenConfirmed,
            approved_amount: Some(BigDecimal::from(10)),
            updated_at: now,
            has_payout_record: false,
            has_wallet_credit_record: false,
            has_notification_record: false,
        })
        .expect("token confirmed without payout should be a mismatch");

        assert_eq!(row.status, "token_confirmed");
        assert_eq!(row.mismatch_type, "needs_payout_record");
        assert_eq!(row.approved_amount, Some("10".to_string()));
    }
}
