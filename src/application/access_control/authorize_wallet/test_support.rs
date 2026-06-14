use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;

#[derive(Default)]
pub(crate) struct FakeWalletAuthorizationStore {
    pub platform_permissions: Vec<Permission>,
    pub organization_permissions: Vec<(i32, Permission)>,
    pub platform_checks: Vec<Permission>,
    pub organization_checks: Vec<(i32, Permission)>,
}

impl WalletAuthorizationStore for FakeWalletAuthorizationStore {
    fn has_platform_permission(
        &mut self,
        _actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        self.platform_checks.push(permission);
        ready(Ok(self.platform_permissions.contains(&permission))).boxed()
    }

    fn has_organization_permission(
        &mut self,
        _actor_user_id: i32,
        organization_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        self.organization_checks.push((organization_id, permission));
        ready(Ok(self
            .organization_permissions
            .contains(&(organization_id, permission))))
        .boxed()
    }
}
