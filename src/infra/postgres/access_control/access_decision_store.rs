use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::authorize_wallet::{
    WalletAuthorizationError, WalletAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::infra::postgres::access_control::permission_checks;

pub struct PostgresAccessDecisionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresAccessDecisionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardAuthorizationStore for PostgresAccessDecisionStore<'_> {
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}

impl WalletAuthorizationStore for PostgresAccessDecisionStore<'_> {
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, WalletAuthorizationError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(|error| WalletAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
