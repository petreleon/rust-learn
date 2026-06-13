use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

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
            user_permission_platform_request(self.conn, actor_user_id, permission.as_str())
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
            user_permission_organization_request(
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
