use bigdecimal::BigDecimal;
use futures::future::BoxFuture;

use crate::application::rewards::decide_amount::{
    RewardAmountDecisionError, RewardAmountDecisionOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardAmountDecision {
    pub actor_user_id: i32,
    pub candidate_id: i64,
    pub target_status: RewardCandidateStatus,
    pub approved_amount: Option<BigDecimal>,
    pub decision_reason: Option<String>,
}

pub trait RewardAmountDecisionStore {
    fn can_approve_reward_amount(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardAmountDecisionError>>;

    fn decide_reward_amount(
        &mut self,
        decision: RewardAmountDecision,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>>;
}
