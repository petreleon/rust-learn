use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::domain::access_control::permissions::Permissions;

#[derive(Default)]
pub(crate) struct FakeWalletAuthorizationStore {
    pub platform_permissions: Vec<Permissions>,
    pub organization_permissions: Vec<(i32, Permissions)>,
    pub platform_checks: Vec<Permissions>,
    pub organization_checks: Vec<(i32, Permissions)>,
}

impl WalletAuthorizationStore for FakeWalletAuthorizationStore {
    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        let permission = permission_from_action(&action);
        let allowed = match scope {
            AccessScope::Platform(_) => {
                self.platform_checks.push(permission);
                self.platform_permissions.contains(&permission)
            }
            AccessScope::Organization(scope) => {
                self.organization_checks
                    .push((scope.organization_id(), permission));
                self.organization_permissions
                    .contains(&(scope.organization_id(), permission))
            }
            AccessScope::Course(_) => false,
        };

        ready(Ok(allowed)).boxed()
    }
}

fn permission_from_action(action: &AccessAction) -> Permissions {
    action.permission_name().parse().unwrap_or_else(|_| {
        panic!(
            "unsupported fake wallet permission: {}",
            action.permission_name()
        )
    })
}
