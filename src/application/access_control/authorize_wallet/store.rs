use futures::future::BoxFuture;

use crate::application::access_control::authorize_wallet::WalletAuthorizationError;
use crate::domain::access_control::permission::Permission;

pub trait WalletAuthorizationStore {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>>;

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>>;
}
