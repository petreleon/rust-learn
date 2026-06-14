use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use std::cmp::Ordering;

use crate::infra::postgres::access_control::{hierarchy_records, permission_checks};

pub(crate) async fn user_permission_platform_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_platform_permission(conn, user_id, permission).await
}

pub(crate) async fn user_permission_course_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_course_permission(conn, user_id, course_id, permission).await
}

pub(crate) async fn user_permission_organization_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_organization_permission(conn, user_id, organization_id, permission).await
}

pub(crate) async fn user_hierarchy_compare_platform(
    conn: &mut AsyncPgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level = hierarchy_records::platform_min_level_for_user(conn, user1_id).await?;
    let user2_top_level = hierarchy_records::platform_min_level_for_user(conn, user2_id).await?;

    compare_top_levels(user1_top_level, user2_top_level)
}

pub(crate) async fn user_hierarchy_compare_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level =
        hierarchy_records::organization_min_level_for_user(conn, user1_id, organization_id).await?;
    let user2_top_level =
        hierarchy_records::organization_min_level_for_user(conn, user2_id, organization_id).await?;

    compare_top_levels(user1_top_level, user2_top_level)
}

fn compare_top_levels(
    user1_top_level: Option<i32>,
    user2_top_level: Option<i32>,
) -> QueryResult<Ordering> {
    match (user1_top_level, user2_top_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater),
        (None, Some(_)) => Ok(Ordering::Less),
    }
}
