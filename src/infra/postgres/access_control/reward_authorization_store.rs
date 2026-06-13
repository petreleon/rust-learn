use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationError, RewardAuthorizationStore,
};
use crate::domain::access_control::permission::Permission;
use crate::repositories::platform_repository::user_permission_platform_request;

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
            user_permission_platform_request(self.conn, actor_user_id, permission.as_str())
                .await
                .map_err(|error| RewardAuthorizationError::PermissionCheck(error.to_string()))
        }
        .boxed()
    }
}
