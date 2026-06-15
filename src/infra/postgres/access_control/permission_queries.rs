use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::infra::postgres::access_control::permission_checks;

pub async fn has_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can(
        conn,
        AccessActor::user(user_id),
        AccessAction::permission(permission),
        AccessScope::platform(),
    )
    .await
}

pub async fn has_course_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can(
        conn,
        AccessActor::user(user_id),
        AccessAction::permission(permission),
        AccessScope::course(course_id),
    )
    .await
}

pub async fn has_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can(
        conn,
        AccessActor::user(user_id),
        AccessAction::permission(permission),
        AccessScope::organization(organization_id),
    )
    .await
}
