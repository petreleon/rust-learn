use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::infra::postgres::access_control::permission_checks;

pub async fn has_platform_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can_platform_permission(conn, user_id, permission).await
}

pub async fn has_course_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can_course_permission(conn, user_id, course_id, permission).await
}

pub async fn has_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::can_organization_permission(conn, user_id, organization_id, permission).await
}
