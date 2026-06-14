use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    PermissionCheckError, PermissionCheckUseCase, PermissionScope,
};
use crate::db::DbPool;
use crate::infra::postgres::access_control::permission_checks;

impl PermissionCheckUseCase for DbPool {
    fn has_permission(
        &self,
        actor_user_id: i32,
        scope: PermissionScope,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, PermissionCheckError>> {
        async move {
            let mut conn = self
                .get()
                .await
                .map_err(|error| PermissionCheckError::Connection(error.to_string()))?;

            match scope {
                PermissionScope::Platform => {
                    permission_checks::has_platform_permission(
                        &mut conn,
                        actor_user_id,
                        &permission,
                    )
                    .await
                }
                PermissionScope::Course { course_id } => {
                    permission_checks::has_course_permission(
                        &mut conn,
                        actor_user_id,
                        course_id,
                        &permission,
                    )
                    .await
                }
                PermissionScope::Organization { organization_id } => {
                    permission_checks::has_organization_permission(
                        &mut conn,
                        actor_user_id,
                        organization_id,
                        &permission,
                    )
                    .await
                }
            }
            .map_err(|error| PermissionCheckError::Query(error.to_string()))
        }
        .boxed()
    }
}
