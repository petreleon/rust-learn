use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

#[derive(Default)]
pub(crate) struct FakeRewardAuthorizationStore {
    pub platform_permissions: Vec<Permission>,
    pub course_permissions: Vec<(i32, Permission)>,
    pub platform_checks: Vec<Permission>,
    pub course_checks: Vec<(i32, Permission)>,
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

    fn has_course_permission(
        &mut self,
        _actor_user_id: i32,
        course_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        self.course_checks.push((course_id, permission));
        ready(Ok(self
            .course_permissions
            .contains(&(course_id, permission))))
        .boxed()
    }
}
