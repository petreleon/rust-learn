use futures::future::BoxFuture;

use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyError, RewardPolicyOutput,
};

pub trait RewardPolicyUseCase: Send + Sync {
    fn create_reward_policy(
        &self,
        actor_user_id: i32,
        command: CreateRewardPolicyCommand,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>>;

    fn list_reward_policies(
        &self,
        actor_user_id: i32,
        query: ListRewardPoliciesQuery,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyOutput>, RewardPolicyError>>;
}
