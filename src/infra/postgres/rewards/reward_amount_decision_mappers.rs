use crate::application::rewards::decide_amount::{
    RewardAmountDecisionError, RewardAmountDecisionOutput,
};
use crate::infra::postgres::rewards::reward_candidate_fraud_blocks::RewardCandidateFraudBlockError;
use crate::infra::postgres::rewards::reward_vocabulary::{
    parse_candidate_source_scope, parse_candidate_status, parse_reward_event_type,
};
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn map_reward_amount_decision_candidate(
    candidate: RewardCandidate,
) -> Result<RewardAmountDecisionOutput, RewardAmountDecisionError> {
    let status = parse_candidate_status(&candidate.status, |message| {
        RewardAmountDecisionError::InvalidStatus(message)
    })?;
    let source_scope = parse_candidate_source_scope(&candidate.source_scope, |message| {
        RewardAmountDecisionError::Database(message)
    })?;
    let event_type = parse_reward_event_type(&candidate.event_type, |message| {
        RewardAmountDecisionError::Database(message)
    })?;

    Ok(RewardAmountDecisionOutput {
        id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_scope,
        source_organization_id: candidate.source_organization_id,
        event_type,
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

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::map_reward_amount_decision_candidate;
    use crate::application::rewards::decide_amount::RewardAmountDecisionError;
    use crate::domain::rewards::candidate::event_type::RewardEventType;
    use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = map_reward_amount_decision_candidate(candidate("amount_approved"))
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::AmountApproved);
        assert_eq!(mapped.source_scope, RewardCandidateSourceScope::Course);
        assert_eq!(mapped.event_type, RewardEventType::CourseCompletion);
        assert_eq!(mapped.approved_amount, Some(BigDecimal::from(10)));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            map_reward_amount_decision_candidate(candidate("not_real")).unwrap_err(),
            RewardAmountDecisionError::InvalidStatus(
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
