use diesel::{dsl::min, prelude::*};
use std::cmp::Ordering;

use crate::db::schema::{
    course_roles, role_course_hierarchy, role_permission_course, user_role_course,
};
use crate::config::constants::roles::Roles;

/// Checks if a user has a specific permission in a course
pub fn user_permission_course_request(
    conn: &mut PgConnection,
    p_user_id: i32,
    p_course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let has_permission = diesel::select(diesel::dsl::exists(
        user_role_course::table
            .inner_join(course_roles::table.on(
                user_role_course::course_role_id.eq(course_roles::id.nullable()),
            ))
            .inner_join(role_permission_course::table.on(
                course_roles::id
                    .nullable()
                    .eq(role_permission_course::course_role_id),
            ))
            .filter(user_role_course::user_id.eq(p_user_id))
            .filter(user_role_course::course_id.eq(p_course_id))
            .filter(role_permission_course::permission.eq(permission)),
    ))
    .get_result(conn)?;

    Ok(has_permission)
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
    let user1_top_level = role_course_hierarchy::table
        .inner_join(user_role_course::table.on(
            role_course_hierarchy::course_role_id.eq(user_role_course::course_role_id),
        ))
        .filter(user_role_course::user_id.eq(user1_id))
        .filter(user_role_course::course_id.eq(course_id))
        .select(min(role_course_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    let user2_top_level = role_course_hierarchy::table
        .inner_join(user_role_course::table.on(
            role_course_hierarchy::course_role_id.eq(user_role_course::course_role_id),
        ))
        .filter(user_role_course::user_id.eq(user2_id))
        .filter(user_role_course::course_id.eq(course_id))
        .select(min(role_course_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

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
    use crate::db::schema::course_roles::dsl::*;
    use crate::db::schema::user_role_course::dsl as urc_dsl;

    // Map enum to the expected role name in course_roles
    // Assumption: course_roles.name stores simple names like "TEACHER" and "STUDENT".
    let role_name: String = match role {
        Roles::TEACHER | Roles::TEACHER_COURSE => "TEACHER".to_string(),
        Roles::STUDENT | Roles::STUDENT_COURSE => "STUDENT".to_string(),
        // Fallback to enum's string for other values if present in course context
        other => other.to_string(),
    };

    // Lookup the course_role_id by name
    let course_role_id_value = course_roles
        .filter(name.eq(role_name))
        .select(id)
        .first::<i32>(conn)?;

    // Insert the user-role assignment into the user_role_course table
    let new_user_role = (
        urc_dsl::user_id.eq(p_user_id),
        urc_dsl::course_role_id.eq(course_role_id_value),
        urc_dsl::course_id.eq(p_course_id),
    );

    diesel::insert_into(user_role_course::table)
        .values(&new_user_role)
        .execute(conn)
}
