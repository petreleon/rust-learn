use crate::application::rewards::plan_payout::{
    RewardPayoutCandidate, RewardPayoutPlanError, RewardPayoutPolicy,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_policy::RewardPolicy;

pub(super) fn map_reward_payout_plan_error(error: diesel::result::Error) -> RewardPayoutPlanError {
    match error {
        diesel::result::Error::NotFound => RewardPayoutPlanError::NoActivePolicy,
        other => RewardPayoutPlanError::Database(other.to_string()),
    }
}

pub(super) fn map_reward_payout_candidate(
    candidate: RewardCandidate,
) -> Result<RewardPayoutCandidate, RewardPayoutPlanError> {
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| RewardPayoutPlanError::InvalidStatus(error.to_string()))?;

    Ok(RewardPayoutCandidate {
        id: candidate.id,
        course_id: candidate.course_id,
        event_type: candidate.event_type,
        status,
        approved_amount: candidate.approved_amount,
    })
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use serde_json::json;

    use super::map_reward_payout_candidate;
    use crate::application::rewards::plan_payout::RewardPayoutPlanError;
    use crate::domain::rewards::candidate::status::RewardCandidateStatus;
    use crate::models::reward_candidate::RewardCandidate;

    #[test]
    fn maps_known_candidate_status_into_domain_status() {
        let mapped = map_reward_payout_candidate(candidate("amount_approved"))
            .expect("known status should map");

        assert_eq!(mapped.status, RewardCandidateStatus::AmountApproved);
        assert_eq!(mapped.approved_amount, Some(BigDecimal::from(10)));
    }

    #[test]
    fn rejects_unknown_candidate_status_at_infra_boundary() {
        assert_eq!(
            map_reward_payout_candidate(candidate("not_real")).unwrap_err(),
            RewardPayoutPlanError::InvalidStatus(
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
            amount_reviewer_user_id: None,
            approved_amount: Some(BigDecimal::from(10)),
            amount_decision_reason: None,
            amount_decided_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl From<RewardPolicy> for RewardPayoutPolicy {
    fn from(policy: RewardPolicy) -> Self {
        Self {
            id: policy.id,
            payment_strategy: policy.payment_strategy,
        }
    }
}
