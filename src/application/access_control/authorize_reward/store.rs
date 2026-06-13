use futures::future::BoxFuture;

use crate::application::access_control::authorize_reward::RewardAuthorizationError;
use crate::domain::access_control::permission::Permission;

pub trait RewardAuthorizationStore {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>>;

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>>;
}
