use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

pub async fn authorize_reward_action(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    match action {
        RewardAuthorizationAction::ApproveRewardAmount => {
            store
                .has_platform_permission(actor_user_id, Permission::ApproveRewardAmount)
                .await
        }
        RewardAuthorizationAction::ExecuteRewardPayout => {
            store
                .has_platform_permission(actor_user_id, Permission::ExecuteRewardPayout)
                .await
        }
        RewardAuthorizationAction::ManageRewardPolicy => {
            store
                .has_platform_permission(actor_user_id, Permission::SetRewardPolicy)
                .await
        }
        RewardAuthorizationAction::RecordRewardCompensation => {
            has_any_platform_permission(
                store,
                actor_user_id,
                &[Permission::ReconcileWallets, Permission::ManageWallets],
            )
            .await
        }
        RewardAuthorizationAction::ViewRewardAudit => {
            store
                .has_platform_permission(actor_user_id, Permission::ViewRewardAudit)
                .await
        }
    }
}

async fn has_any_platform_permission(
    store: &mut impl RewardAuthorizationStore,
    actor_user_id: i32,
    permissions: &[Permission],
) -> Result<bool, RewardAuthorizationError> {
    for &permission in permissions {
        if store
            .has_platform_permission(actor_user_id, permission)
            .await?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
