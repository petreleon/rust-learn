use futures::future::BoxFuture;

use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyAuditEventOutput,
    RewardPolicyError, RewardPolicyOutput, UpdateRewardPolicyActivationCommand,
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

    fn update_reward_policy_activation(
        &self,
        actor_user_id: i32,
        command: UpdateRewardPolicyActivationCommand,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>>;

    fn list_reward_policy_audit(
        &self,
        actor_user_id: i32,
        policy_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyAuditEventOutput>, RewardPolicyError>>;
}
