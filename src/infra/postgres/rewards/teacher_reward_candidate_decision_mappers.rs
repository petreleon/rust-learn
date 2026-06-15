use crate::application::rewards::decide_teacher_candidate::{
    TeacherRewardCandidateDecisionError, TeacherRewardCandidateDecisionOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::RewardCandidateFraudBlockError;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn map_teacher_decision_candidate(
    candidate: RewardCandidate,
) -> Result<TeacherRewardCandidateDecisionOutput, TeacherRewardCandidateDecisionError> {
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| TeacherRewardCandidateDecisionError::InvalidStatus(error.to_string()))?;

    Ok(TeacherRewardCandidateDecisionOutput {
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
        approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
        amount_decision_reason: candidate.amount_decision_reason,
        amount_decided_at: candidate.amount_decided_at,
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    })
}

pub(super) fn map_teacher_decision_error(
    error: diesel::result::Error,
) -> TeacherRewardCandidateDecisionError {
    match error {
        diesel::result::Error::NotFound => TeacherRewardCandidateDecisionError::NotFound,
        other => TeacherRewardCandidateDecisionError::Database(other.to_string()),
    }
}

impl From<RewardCandidateFraudBlockError> for TeacherRewardCandidateDecisionError {
    fn from(error: RewardCandidateFraudBlockError) -> Self {
        match error {
            RewardCandidateFraudBlockError::Blocked(message) => Self::InvalidStatus(message),
            RewardCandidateFraudBlockError::NotFound => Self::NotFound,
            RewardCandidateFraudBlockError::Database(message) => Self::Database(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::map_teacher_decision_candidate;
    use crate::application::rewards::decide_teacher_candidate::TeacherRewardCandidateDecisionError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = map_teacher_decision_candidate(candidate("teacher_approved"))
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::TeacherApproved);
        assert_eq!(mapped.approved_amount, Some("10".to_string()));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            map_teacher_decision_candidate(candidate("not_real")).unwrap_err(),
            TeacherRewardCandidateDecisionError::InvalidStatus(
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
            amount_reviewer_user_id: None,
            approved_amount: Some(BigDecimal::from(10)),
            amount_decision_reason: None,
            amount_decided_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
