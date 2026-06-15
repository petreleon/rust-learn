use crate::application::rewards::submit_candidate::{
    RewardCandidateSubmissionError, RewardCandidateSubmissionOutput,
};
use crate::domain::rewards::candidate::evidence::RewardEvidenceError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::RewardCandidateFraudBlockError;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn map_reward_candidate_submission(
    candidate: RewardCandidate,
) -> Result<RewardCandidateSubmissionOutput, RewardCandidateSubmissionError> {
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| RewardCandidateSubmissionError::InvalidStatus(error.to_string()))?;

    Ok(RewardCandidateSubmissionOutput {
        id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_scope: candidate.source_scope,
        source_organization_id: candidate.source_organization_id,
        event_type: candidate.event_type,
        idempotency_key: candidate.idempotency_key,
        evidence: candidate.evidence,
        status,
        teacher_approver_user_id: candidate.teacher_approver_user_id,
        teacher_decision_reason: candidate.teacher_decision_reason,
        teacher_decided_at: candidate.teacher_decided_at,
        amount_reviewer_user_id: candidate.amount_reviewer_user_id,
        approved_amount: candidate.approved_amount,
        amount_decision_reason: candidate.amount_decision_reason,
        amount_decided_at: candidate.amount_decided_at,
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    })
}

pub(super) fn map_reward_candidate_submission_error(
    error: diesel::result::Error,
) -> RewardCandidateSubmissionError {
    match error {
        diesel::result::Error::NotFound => RewardCandidateSubmissionError::NotFound,
        other => RewardCandidateSubmissionError::Database(other.to_string()),
    }
}

impl From<RewardCandidateFraudBlockError> for RewardCandidateSubmissionError {
    fn from(error: RewardCandidateFraudBlockError) -> Self {
        match error {
            RewardCandidateFraudBlockError::Blocked(message) => Self::InvalidStatus(message),
            RewardCandidateFraudBlockError::NotFound => Self::NotFound,
            RewardCandidateFraudBlockError::Database(message) => Self::Database(message),
        }
    }
}

impl From<RewardEvidenceError> for RewardCandidateSubmissionError {
    fn from(error: RewardEvidenceError) -> Self {
        match error {
            RewardEvidenceError::MissingNumber { key } => {
                Self::InvalidInput(format!("{} evidence is required", key))
            }
            RewardEvidenceError::BelowThreshold { key } => {
                Self::InvalidInput(format!("{} evidence is below the reward threshold", key))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::map_reward_candidate_submission;
    use crate::application::rewards::submit_candidate::RewardCandidateSubmissionError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = map_reward_candidate_submission(candidate("pending_teacher_approval"))
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::PendingTeacherApproval);
        assert_eq!(mapped.approved_amount, Some(BigDecimal::from(10)));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            map_reward_candidate_submission(candidate("not_real")).unwrap_err(),
            RewardCandidateSubmissionError::InvalidStatus(
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
            teacher_approver_user_id: None,
            teacher_decision_reason: None,
            teacher_decided_at: None,
            amount_reviewer_user_id: Some(6),
            approved_amount: Some(BigDecimal::from(10)),
            amount_decision_reason: Some("ok".to_string()),
            amount_decided_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
