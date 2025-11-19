// src/utils/db_utils/platform_permission_utils.rs

use diesel::prelude::*;
use diesel::insert_into;
use diesel::QueryResult;
use diesel::dsl::{exists, select};
use crate::models::role_permission_platform::RolePermissionPlatform;
use crate::models::role::PlatformRole;
use crate::config::constants::roles::Roles;
use crate::config::constants::permissions::Permissions;

/// Assigns a permission to a platform role after checking if it already exists.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `role` - A role variant from the `Roles` enum.
/// * `perm` - A permission variant from the `Permissions` enum.
///
/// # Returns
///
/// A `QueryResult<usize>` indicating the number of rows affected.  
/// Returns 0 if the permission is already assigned.
pub fn assign_permission_to_role_platform(
    conn: &mut PgConnection,
    role: Roles,
    perm: Permissions,
) -> QueryResult<usize> {
    // Retrieve the platform_role_id based on the role name.
    let platform_role_id_value = PlatformRole::find_by_name(&role.to_string(), conn)?;

    // Convert the permission enum to a string.
    let perm_str = perm.to_string();

    // Assign permission to role
    RolePermissionPlatform::assign(conn, platform_role_id_value, &perm_str)
}
