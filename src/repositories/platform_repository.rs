use diesel::prelude::*;
use diesel::{dsl::min, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::cmp::Ordering;

use crate::models::user_role_platform::UserRolePlatform;
use crate::models::role_platform_hierarchy::RolePlatformHierarchy;
use crate::models::role::PlatformRole;
use crate::config::constants::roles::Roles;

// Checks if a user has a specific permission on the platform
pub async fn user_permission_platform_request(
    conn: &mut AsyncPgConnection,
    p_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    UserRolePlatform::has_permission(conn, p_user_id, permission).await
}

// Compares the hierarchy level of two users on the platform
pub async fn user_hierarchy_compare_platform(
    conn: &mut AsyncPgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {

    let user1_max_level = RolePlatformHierarchy::get_min_level(conn, user1_id).await?;
    let user2_max_level = RolePlatformHierarchy::get_min_level(conn, user2_id).await?;

    match (user1_max_level, user2_max_level) {
        (Some(level1), Some(level2)) => Ok(level2.cmp(&level1)),
        (None, None) => Ok(Ordering::Equal),
        (Some(_), None) => Ok(Ordering::Greater), // User with a role is considered 'higher'
        (None, Some(_)) => Ok(Ordering::Less),
    }
}

// Assigns a platform role to a user based on the Roles enum
pub async fn assign_role_to_user(
    conn: &mut AsyncPgConnection,
    p_user_id: i32,
    role: Roles,
) -> QueryResult<usize> {
    // Find the platform role ID from the database based on the role name
    let platform_role_id_value = PlatformRole::find_by_name(&role.to_string(), conn).await?;

    // Insert the user-role assignment into the user_role_platform table
    UserRolePlatform::assign(conn, p_user_id, platform_role_id_value).await
}
