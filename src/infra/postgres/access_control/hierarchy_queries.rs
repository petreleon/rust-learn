use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use std::cmp::Ordering;

use crate::domain::access_control::hierarchy::compare_hierarchy_levels;
use crate::infra::postgres::access_control::hierarchy_records;

pub async fn compare_platform_users(
    conn: &mut AsyncPgConnection,
    first_user_id: i32,
    second_user_id: i32,
) -> QueryResult<Ordering> {
    let first_level = hierarchy_records::platform_min_level_for_user(conn, first_user_id).await?;
    let second_level = hierarchy_records::platform_min_level_for_user(conn, second_user_id).await?;

    Ok(compare_hierarchy_levels(first_level, second_level))
}

pub async fn compare_organization_users(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    first_user_id: i32,
    second_user_id: i32,
) -> QueryResult<Ordering> {
    let first_level =
        hierarchy_records::organization_min_level_for_user(conn, first_user_id, organization_id)
            .await?;
    let second_level =
        hierarchy_records::organization_min_level_for_user(conn, second_user_id, organization_id)
            .await?;

    Ok(compare_hierarchy_levels(first_level, second_level))
}

pub async fn compare_course_users(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    first_user_id: i32,
    second_user_id: i32,
) -> QueryResult<Ordering> {
    let first_level =
        hierarchy_records::course_min_level_for_user(conn, first_user_id, course_id).await?;
    let second_level =
        hierarchy_records::course_min_level_for_user(conn, second_user_id, course_id).await?;

    Ok(compare_hierarchy_levels(first_level, second_level))
}
