use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
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
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        async move {
            let permission = action.permission_name();
            match scope {
                AccessScope::Platform => {
                    has_platform_permission(self.conn, actor.user_id, permission).await
                }
                AccessScope::Organization { organization_id } => {
                    has_organization_permission(
                        self.conn,
                        actor.user_id,
                        organization_id,
                        permission,
                    )
                    .await
                }
                AccessScope::Course { .. } => Ok(false),
            }
            .map_err(|error| WalletAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
