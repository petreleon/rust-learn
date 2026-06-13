use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

#[derive(Default)]
pub(crate) struct FakeRewardAuthorizationStore {
    pub platform_permissions: Vec<Permission>,
    pub platform_checks: Vec<Permission>,
}

impl RewardAuthorizationStore for FakeRewardAuthorizationStore {
    fn has_platform_permission(
        &mut self,
        _actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        self.platform_checks.push(permission);
        ready(Ok(self.platform_permissions.contains(&permission))).boxed()
    }
}
