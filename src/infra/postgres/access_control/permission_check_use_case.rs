use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionError, AccessDecisionUseCase, AccessScope,
};
use crate::db::DbPool;
use crate::infra::postgres::access_control::permission_checks;

impl AccessDecisionUseCase for DbPool {
    fn can(
        &self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, AccessDecisionError>> {
        async move {
            let mut conn = self
                .get()
                .await
                .map_err(|error| AccessDecisionError::Connection(error.to_string()))?;

            let permission = action.permission_name();
            match scope {
                AccessScope::Platform => {
                    permission_checks::has_platform_permission(&mut conn, actor.user_id, permission)
                        .await
                }
                AccessScope::Course { course_id } => {
                    permission_checks::has_course_permission(
                        &mut conn,
                        actor.user_id,
                        course_id,
                        permission,
                    )
                    .await
                }
                AccessScope::Organization { organization_id } => {
                    permission_checks::has_organization_permission(
                        &mut conn,
                        actor.user_id,
                        organization_id,
                        permission,
                    )
                    .await
                }
            }
            .map_err(|error| AccessDecisionError::Query(error.to_string()))
        }
        .boxed()
    }
}
