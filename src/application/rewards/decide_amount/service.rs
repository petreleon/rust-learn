use futures::future::BoxFuture;

use crate::application::rewards::decide_amount::{
    RewardAmountDecisionCommand, RewardAmountDecisionError, RewardAmountDecisionOutput,
};

pub trait RewardAmountDecisionUseCase: Send + Sync {
    fn decide_reward_amount(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
        command: RewardAmountDecisionCommand,
    ) -> BoxFuture<'_, Result<RewardAmountDecisionOutput, RewardAmountDecisionError>>;
}
