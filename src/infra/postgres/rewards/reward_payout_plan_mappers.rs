use crate::application::rewards::plan_payout::{
    RewardPayoutCandidate, RewardPayoutPlanError, RewardPayoutPolicy,
};
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_policy::RewardPolicy;

pub(super) fn map_reward_payout_plan_error(error: diesel::result::Error) -> RewardPayoutPlanError {
    match error {
        diesel::result::Error::NotFound => RewardPayoutPlanError::NoActivePolicy,
        other => RewardPayoutPlanError::Database(other.to_string()),
    }
}

impl From<RewardCandidate> for RewardPayoutCandidate {
    fn from(candidate: RewardCandidate) -> Self {
        Self {
            id: candidate.id,
            course_id: candidate.course_id,
            event_type: candidate.event_type,
            status: candidate.status,
            approved_amount: candidate.approved_amount,
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
