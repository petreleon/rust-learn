use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::infra::postgres::access_control::permission_checks::{
    has_course_permission, has_organization_permission, has_platform_permission,
};

pub struct PostgresRewardAuthorizationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardAuthorizationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardAuthorizationStore for PostgresRewardAuthorizationStore<'_> {
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        async move {
            let permission = action.permission_name();
            match scope {
                AccessScope::Platform(_) => {
                    has_platform_permission(self.conn, actor.user_id, permission).await
                }
                AccessScope::Course(scope) => {
                    has_course_permission(self.conn, actor.user_id, scope.course_id(), permission)
                        .await
                }
                AccessScope::Organization(scope) => {
                    has_organization_permission(
                        self.conn,
                        actor.user_id,
                        scope.organization_id(),
                        permission,
                    )
                    .await
                }
            }
            .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
