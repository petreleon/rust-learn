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
        RewardAuthorizationAction::ExecuteRewardPayout => {
            store
                .has_platform_permission(actor_user_id, Permission::ExecuteRewardPayout)
                .await
        }
    }
}
