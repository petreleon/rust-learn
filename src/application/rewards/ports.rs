use futures::future::BoxFuture;

use crate::application::rewards::manage_reward_policy::{
    RewardPolicyDraft, RewardPolicyError, RewardPolicyListFilter, RewardPolicyOutput,
};

pub trait RewardPolicyStore {
    fn can_manage_reward_policies(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPolicyError>>;

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<(), RewardPolicyError>>;

    fn course_exists(&mut self, course_id: i32) -> BoxFuture<'_, Result<(), RewardPolicyError>>;

    fn create_versioned_policy(
        &mut self,
        draft: RewardPolicyDraft,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>>;

    fn list_policies(
        &mut self,
        filter: RewardPolicyListFilter,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyOutput>, RewardPolicyError>>;
}
