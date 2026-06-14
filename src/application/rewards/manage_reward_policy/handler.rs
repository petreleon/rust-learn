use crate::application::rewards::manage_reward_policy::validation::{
    validated_draft, validated_filter,
};
use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyError, RewardPolicyOutput,
};
use crate::application::rewards::ports::RewardPolicyStore;

pub async fn create_reward_policy(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
    command: CreateRewardPolicyCommand,
) -> Result<RewardPolicyOutput, RewardPolicyError> {
    ensure_can_manage(store, actor_user_id).await?;
    let draft = validated_draft(store, actor_user_id, command).await?;
    store.create_versioned_policy(draft).await
}

pub async fn list_reward_policies(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
    query: ListRewardPoliciesQuery,
) -> Result<Vec<RewardPolicyOutput>, RewardPolicyError> {
    ensure_can_manage(store, actor_user_id).await?;
    store.list_policies(validated_filter(query)?).await
}

async fn ensure_can_manage(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
) -> Result<(), RewardPolicyError> {
    if store.can_manage_reward_policies(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardPolicyError::PermissionDenied(
            "SET_REWARD_POLICY".to_string(),
        ))
    }
}
