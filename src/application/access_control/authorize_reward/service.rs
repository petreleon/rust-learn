use futures::future::BoxFuture;

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError,
};

pub trait RewardAuthorizationUseCase: Send + Sync {
    fn authorize_reward_action(
        &self,
        actor_user_id: i32,
        action: RewardAuthorizationAction,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>>;
}
