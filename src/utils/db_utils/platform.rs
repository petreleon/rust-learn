// src/utils/db_utils/platform.rs
use diesel::prelude::*;
use diesel::{dsl::min, QueryResult};
use std::cmp::Ordering;

// Importing schema modules
use crate::db::schema::{roles, user_role_platform, role_platform_hierarchy, user_role_platform::dsl::*};
use crate::config::constants::roles::Roles;

// Checks if a user has a specific permission on the platform
pub fn user_permission_platform_request(
    conn: &mut PgConnection,
    user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let has_permission = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(user_role_platform::role_id.eq(role_permission_platform::role_id)))
            .filter(user_role_platform::user_id.eq(user_id))
            .filter(role_permission_platform::permission.eq(permission))
    ))
    .get_result(conn)?;

    Ok(has_permission)
}

// Compares the hierarchy level of two users on the platform
pub fn user_hierarchy_compare_platform(
    conn: &mut PgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {

    let user1_max_level = role_platform_hierarchy::table
        .inner_join(user_role_platform::table.on(role_platform_hierarchy::role_id.eq(user_role_platform::role_id)))
        .filter(user_role_platform::user_id.eq(user1_id))
        .select(min(role_platform_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    let user2_max_level = role_platform_hierarchy::table
        .inner_join(user_role_platform::table.on(role_platform_hierarchy::role_id.eq(user_role_platform::role_id)))
        .filter(user_role_platform::user_id.eq(user2_id))
        .select(min(role_platform_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)?;

    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater), // User with a role is considered 'higher'
        (None, Some(_)) => Ok(Ordering::Less),
    }
}

// Assigns a role to a user based on the Roles enum
pub fn assign_role_to_user(
    conn: &mut PgConnection,
    user_id: i32,
    role: Roles,
) -> QueryResult<usize> {
    use roles::dsl::*;

    // Find the role ID from the database based on the role name
    let role_id_value = roles
        .filter(name.eq(role.to_string()))
        .select(id)
        .first::<i32>(conn)?;

    // Insert the user-role assignment into the user_role_platform table
    let new_user_role = (
        user_id.eq(user_id),
        role_id.eq(role_id_value),
    );

    diesel::insert_into(user_role_platform::table)
        .values(&new_user_role)
        .execute(conn)
}
