use diesel::{dsl::min, prelude::*};
use std::cmp::Ordering;

use crate::models::user_role_course::UserRoleCourse;
use crate::models::role_course_hierarchy::RoleCourseHierarchy;
use crate::models::role::CourseRole;
use crate::config::constants::roles::Roles;

/// Checks if a user has a specific permission in a course
pub fn user_permission_course_request(
    conn: &mut PgConnection,
    p_user_id: i32,
    p_course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    UserRoleCourse::has_permission(conn, p_user_id, p_course_id, permission)
}

/// Compares the hierarchy of two users in a course
/// Lower hierarchy_level means higher privilege (0 is highest).
/// Returns Ordering::Greater if user1 outranks user2 (mirrors organization/platform utils).
pub fn user_hierarchy_compare_course(
    conn: &mut PgConnection,
    course_id: i32,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_top_level = RoleCourseHierarchy::get_min_level(conn, user1_id, course_id)?;
    let user2_top_level = RoleCourseHierarchy::get_min_level(conn, user2_id, course_id)?;

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
pub fn assign_role_to_user_in_course(
    conn: &mut PgConnection,
    p_user_id: i32,
    p_course_id: i32,
    role: Roles,
) -> QueryResult<usize> {
    // Map enum to the expected role name in course_roles
    // Assumption: course_roles.name stores simple names like "TEACHER" and "STUDENT".
    let role_name: String = match role {
        Roles::TEACHER | Roles::TEACHER_COURSE => "TEACHER".to_string(),
        Roles::STUDENT | Roles::STUDENT_COURSE => "STUDENT".to_string(),
        // Fallback to enum's string for other values if present in course context
        other => other.to_string(),
    };

    // Lookup the course_role_id by name
    let course_role_id_value = CourseRole::find_by_name(&role_name, conn)?;

    // Insert the user-role assignment into the user_role_course table
    UserRoleCourse::assign(conn, p_user_id, p_course_id, course_role_id_value)
}
