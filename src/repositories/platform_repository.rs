use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use std::cmp::Ordering;

use crate::config::constants::roles::Roles;
use crate::infra::postgres::access_control::hierarchy_records;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::access_control::platform_role_records;
use crate::infra::postgres::access_control::role_catalog_store;

// Checks if a user has a specific permission on the platform
pub async fn user_permission_platform_request(
    conn: &mut AsyncPgConnection,
    p_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_checks::has_platform_permission(conn, p_user_id, permission).await
}

// Compares the hierarchy level of two users on the platform
pub async fn user_hierarchy_compare_platform(
    conn: &mut AsyncPgConnection,
    user1_id: i32,
    user2_id: i32,
) -> QueryResult<Ordering> {
    let user1_max_level = hierarchy_records::platform_min_level_for_user(conn, user1_id).await?;
    let user2_max_level = hierarchy_records::platform_min_level_for_user(conn, user2_id).await?;

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
    let platform_role_id_value =
        role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;

    // Insert the user-role assignment into the user_role_platform table
    platform_role_records::assign_platform_role_to_user(conn, p_user_id, platform_role_id_value)
        .await
}

pub async fn assign_role_to_user_with_hierarchy(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    target_user_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    let assigner_level = hierarchy_records::platform_min_level_for_user(conn, assigner_id)
        .await?
        .ok_or(diesel::result::Error::NotFound)?;

    let target_level_opt =
        hierarchy_records::platform_min_level_for_user(conn, target_user_id).await?;

    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name).await?;
    let target_role_level = hierarchy_records::platform_role_level(conn, role_id).await?;

    if assigner_level >= target_role_level {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    if let Some(target_level) = target_level_opt {
        if assigner_level >= target_level {
            return Err(diesel::result::Error::RollbackTransaction);
        }
    }

    platform_role_records::assign_platform_role_to_user(conn, target_user_id, role_id).await
}
