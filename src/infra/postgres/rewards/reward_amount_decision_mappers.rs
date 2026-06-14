use crate::application::rewards::decide_amount::{
    RewardAmountDecisionError, RewardAmountDecisionOutput,
};
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::RewardCandidateFraudBlockError;
use crate::models::reward_candidate::RewardCandidate;

impl From<RewardCandidate> for RewardAmountDecisionOutput {
    fn from(candidate: RewardCandidate) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            student_user_id: candidate.student_user_id,
            submitter_user_id: candidate.submitter_user_id,
            source_scope: candidate.source_scope,
            source_organization_id: candidate.source_organization_id,
            event_type: candidate.event_type,
            idempotency_key: candidate.idempotency_key,
            evidence: candidate.evidence,
            status: candidate.status,
            teacher_approver_user_id: candidate.teacher_approver_user_id,
            teacher_decision_reason: candidate.teacher_decision_reason,
            teacher_decided_at: candidate.teacher_decided_at,
            amount_reviewer_user_id: candidate.amount_reviewer_user_id,
            approved_amount: candidate.approved_amount,
            amount_decision_reason: candidate.amount_decision_reason,
            amount_decided_at: candidate.amount_decided_at,
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}

pub(super) fn map_reward_amount_decision_error(
    error: diesel::result::Error,
) -> RewardAmountDecisionError {
    match error {
        diesel::result::Error::NotFound => RewardAmountDecisionError::NotFound,
        other => RewardAmountDecisionError::Database(other.to_string()),
    }
}

impl From<RewardCandidateFraudBlockError> for RewardAmountDecisionError {
    fn from(error: RewardCandidateFraudBlockError) -> Self {
        match error {
            RewardCandidateFraudBlockError::Blocked(message) => Self::InvalidStatus(message),
            RewardCandidateFraudBlockError::NotFound => Self::NotFound,
            RewardCandidateFraudBlockError::Database(message) => Self::Database(message),
        }
    }
}
