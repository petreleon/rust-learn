use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;
use crate::infra::postgres::access_control::permission_checks::{
    has_organization_permission, has_platform_permission,
};

pub struct PostgresWalletAuthorizationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletAuthorizationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletAuthorizationStore for PostgresWalletAuthorizationStore<'_> {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        async move {
            has_platform_permission(self.conn, actor_user_id, permission.as_str())
                .await
                .map_err(|error| WalletAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        async move {
            has_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                permission.as_str(),
            )
            .await
            .map_err(|error| WalletAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
