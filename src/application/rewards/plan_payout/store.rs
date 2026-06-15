use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::rewards::plan_payout::RewardPayoutPlanError;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutCandidate {
    pub id: i64,
    pub course_id: i32,
    pub event_type: String,
    pub status: RewardCandidateStatus,
    pub approved_amount: Option<BigDecimal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RewardPayoutPolicy {
    pub id: i64,
    pub payment_strategy: String,
}

pub trait RewardPayoutPlanStore {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>>;

    fn load_candidate(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutCandidate, RewardPayoutPlanError>>;

    fn active_policy_for_candidate(
        &mut self,
        course_id: i32,
        event_type: String,
    ) -> BoxFuture<'_, Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError>>;

    fn has_presigner_contract(&mut self) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>>;
}
