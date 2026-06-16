use crate::application::rewards::manage_reward_policy::validation::{
    validated_draft, validated_filter,
};
use crate::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyAuditEventOutput,
    RewardPolicyError, RewardPolicyOutput, UpdateRewardPolicyActivationCommand,
};
use crate::application::rewards::ports::RewardPolicyStore;
use crate::domain::access_control::permissions::Permissions;

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

pub async fn update_reward_policy_activation(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
    command: UpdateRewardPolicyActivationCommand,
) -> Result<RewardPolicyOutput, RewardPolicyError> {
    ensure_can_manage(store, actor_user_id).await?;
    ensure_positive_policy_id(command.policy_id)?;
    store.update_policy_activation(actor_user_id, command).await
}

pub async fn list_reward_policy_audit(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
    policy_id: i64,
) -> Result<Vec<RewardPolicyAuditEventOutput>, RewardPolicyError> {
    ensure_can_manage(store, actor_user_id).await?;
    ensure_positive_policy_id(policy_id)?;
    store.reward_policy_exists(policy_id).await?;
    store.list_policy_audit_events(policy_id).await
}

async fn ensure_can_manage(
    store: &mut impl RewardPolicyStore,
    actor_user_id: i32,
) -> Result<(), RewardPolicyError> {
    if store.can_manage_reward_policies(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardPolicyError::PermissionDenied(
            Permissions::SET_REWARD_POLICY.into(),
        ))
    }
}

fn ensure_positive_policy_id(policy_id: i64) -> Result<(), RewardPolicyError> {
    if policy_id > 0 {
        Ok(())
    } else {
        Err(RewardPolicyError::InvalidInput(
            "reward policy id must be positive".to_string(),
        ))
    }
}
