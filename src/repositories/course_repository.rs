use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use std::cmp::Ordering;

use crate::models::role_course_hierarchy::RoleCourseHierarchy;
use crate::models::user_role_course::UserRoleCourse;
use crate::repositories::delegated_permission_repository;

/// Checks if a user has a specific permission in a course
pub async fn user_permission_course_request(
    conn: &mut AsyncPgConnection,
    p_user_id: i32,
    p_course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    if UserRoleCourse::has_permission(conn, p_user_id, p_course_id, permission).await? {
        return Ok(true);
    }

    let has_delegation = delegated_permission_repository::has_active_course_delegation(
        conn,
        p_user_id,
        p_course_id,
        permission,
    )
    .await?;
    if has_delegation {
        log::info!(
            "event=delegated_permission_used scope=course user_id={} course_id={} permission={}",
            p_user_id,
            p_course_id,
            permission
        );
    }

    Ok(has_delegation)
}

/// Compares the hierarchy of two users in a course
/// Lower hierarchy_level means higher privilege (0 is highest).
/// Returns Ordering::Greater if user1 outranks user2 (mirrors organization/platform utils).
pub async fn user_hierarchy_compare_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level = RoleCourseHierarchy::get_min_level(conn, user1_id, course_id).await?;
    let user2_top_level = RoleCourseHierarchy::get_min_level(conn, user2_id, course_id).await?;

    match (user1_top_level, user2_top_level) {
        // Reverse compare so smaller number (higher privilege) wins
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        // Having any role in the course outranks having none
        (Some(_), None) => Ok(Ordering::Greater),
        (None, Some(_)) => Ok(Ordering::Less),
    }
}
