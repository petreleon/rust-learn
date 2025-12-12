use diesel::{dsl::min, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::cmp::Ordering;

use crate::models::user_role_course::UserRoleCourse;
use crate::models::role_course_hierarchy::RoleCourseHierarchy;
use crate::models::role::CourseRole;

/// Checks if a user has a specific permission in a course
pub async fn user_permission_course_request(
    conn: &mut AsyncPgConnection,
    p_user_id: i32,
    p_course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    UserRoleCourse::has_permission(conn, p_user_id, p_course_id, permission).await
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

/// Assigns a course role to a user for a specific course
pub async fn assign_role_to_user_in_course(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    p_user_id: i32,
    p_course_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    // 1. Get Assigner's Hierarchy Level
    let assigner_level = RoleCourseHierarchy::get_min_level(conn, assigner_id, p_course_id).await?
        .ok_or(diesel::result::Error::NotFound)?; // Assigner must have a role in the course

    // 2. Get Assignee's (Target User) Hierarchy Level
    let assignee_level_opt = RoleCourseHierarchy::get_min_level(conn, p_user_id, p_course_id).await?;

    // 3. Lookup the course_role_id by name
    let role_id = CourseRole::find_by_name(role_name, conn).await?;

    // 4. Get Target Role's Hierarchy Level
    let target_role_level = RoleCourseHierarchy::get_role_level(conn, role_id).await?;

    // 5. Enforce Hierarchy Rules: Lower value means higher rank (0 is highest)
    
    // Rule A: Assigner must be higher rank than the role they are assigning
    if assigner_level >= target_role_level {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    // Rule B: Assigner must be higher rank than the user they are assigning to (if user already has a role)
    if let Some(assignee_level) = assignee_level_opt {
        if assigner_level >= assignee_level {
             return Err(diesel::result::Error::RollbackTransaction);
        }
    }

    // Insert the user-role assignment into the user_role_course table
    UserRoleCourse::assign(conn, p_user_id, p_course_id, role_id).await
}
