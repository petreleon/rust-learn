use futures::future::BoxFuture;

use crate::application::rewards::plan_payout::{RewardPayoutPlan, RewardPayoutPlanError};

pub trait RewardPayoutPlanUseCase: Send + Sync {
    fn plan_reward_payout(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutPlan, RewardPayoutPlanError>>;

    fn plan_reward_payout_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutPlan, RewardPayoutPlanError>>;
}
