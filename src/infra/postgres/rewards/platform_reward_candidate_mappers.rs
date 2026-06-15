use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateRecord, PlatformRewardCandidatesError,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::models::reward_candidate::RewardCandidate;

pub(super) fn map_platform_reward_candidate_record(
    candidate: RewardCandidate,
) -> Result<PlatformRewardCandidateRecord, PlatformRewardCandidatesError> {
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| PlatformRewardCandidatesError::InvalidStatus(error.to_string()))?;

    Ok(PlatformRewardCandidateRecord {
        id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_scope: candidate.source_scope,
        source_organization_id: candidate.source_organization_id,
        event_type: candidate.event_type,
        status,
        teacher_approver_user_id: candidate.teacher_approver_user_id,
        teacher_decision_reason: candidate.teacher_decision_reason,
        approved_amount: candidate.approved_amount.map(|amount| amount.to_string()),
        created_at: candidate.created_at,
        updated_at: candidate.updated_at,
    })
}

pub(super) fn map_platform_reward_candidate_error(
    error: diesel::result::Error,
) -> PlatformRewardCandidatesError {
    PlatformRewardCandidatesError::Database(error.to_string())
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::map_platform_reward_candidate_record;
    use crate::application::rewards::list_platform_candidates::PlatformRewardCandidatesError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = map_platform_reward_candidate_record(candidate("amount_approved"))
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::AmountApproved);
        assert_eq!(mapped.approved_amount, Some("10".to_string()));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            map_platform_reward_candidate_record(candidate("not_real")).unwrap_err(),
            PlatformRewardCandidatesError::InvalidStatus(
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
