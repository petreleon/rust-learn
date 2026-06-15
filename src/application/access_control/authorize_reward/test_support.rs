use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permissions::Permissions;

#[derive(Default)]
pub(crate) struct FakeRewardAuthorizationStore {
    pub platform_permissions: Vec<Permissions>,
    pub course_permissions: Vec<(i32, Permissions)>,
    pub organization_permissions: Vec<(i32, Permissions)>,
    pub platform_checks: Vec<Permissions>,
    pub course_checks: Vec<(i32, Permissions)>,
    pub organization_checks: Vec<(i32, Permissions)>,
}

impl RewardAuthorizationStore for FakeRewardAuthorizationStore {
    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        let permission = permission_from_action(&action);
        let allowed = match scope {
            AccessScope::Platform(_) => {
                self.platform_checks.push(permission);
                self.platform_permissions.contains(&permission)
            }
            AccessScope::Course(scope) => {
                self.course_checks.push((scope.course_id(), permission));
                self.course_permissions
                    .contains(&(scope.course_id(), permission))
            }
            AccessScope::Organization(scope) => {
                self.organization_checks
                    .push((scope.organization_id(), permission));
                self.organization_permissions
                    .contains(&(scope.organization_id(), permission))
            }
        };

        ready(Ok(allowed)).boxed()
    }
}

fn permission_from_action(action: &AccessAction) -> Permissions {
    action.permission_name().parse().unwrap_or_else(|_| {
        panic!(
            "unsupported fake reward permission: {}",
            action.permission_name()
        )
    })
}
