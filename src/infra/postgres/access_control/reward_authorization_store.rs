use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;
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
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        async move {
            has_platform_permission(self.conn, actor_user_id, permission.as_str())
                .await
                .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        async move {
            has_course_permission(self.conn, actor_user_id, course_id, permission.as_str())
                .await
                .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: Permission,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>> {
        async move {
            has_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                permission.as_str(),
            )
            .await
            .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
