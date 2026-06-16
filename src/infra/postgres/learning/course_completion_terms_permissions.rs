use diesel_async::AsyncPgConnection;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::manage_course_completion_terms::CourseCompletionTermsError;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::learning::course_completion_terms_records;

pub async fn has_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> Result<bool, CourseCompletionTermsError> {
    let actor = AccessActor::user(actor_user_id);
    if can(conn, actor, permission, AccessScope::course(course_id)).await?
        || can(conn, actor, permission, AccessScope::platform()).await?
    {
        return Ok(true);
    }

    let context = course_completion_terms_records::course_context(conn, course_id).await?;
    if let Some(organization_id) = context.organization_id {
        return can(
            conn,
            actor,
            permission,
            AccessScope::organization(organization_id),
        )
        .await;
    }
    Ok(false)
}

async fn can(
    conn: &mut AsyncPgConnection,
    actor: AccessActor,
    permission: &str,
    scope: AccessScope,
) -> Result<bool, CourseCompletionTermsError> {
    permission_checks::can(
        conn,
        actor,
        AccessAction::permission(permission.to_string()),
        scope,
    )
    .await
    .map_err(|error| CourseCompletionTermsError::Database(error.to_string()))
}
