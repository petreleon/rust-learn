// src/utils/db_utils/platform_permission_utils.rs

use diesel::prelude::*;
use diesel::insert_into;
use diesel::QueryResult;
use diesel::dsl::{exists, select};
use crate::db::schema::role_permission_platform;
use crate::db::schema::role_permission_platform::dsl::*;
use crate::db::schema::platform_roles::dsl::{platform_roles, name as role_name, id as platform_role_id_column};
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
    let platform_role_id_value = platform_roles
        .filter(role_name.eq(role.to_string()))
        .select(platform_role_id_column)
        .first::<i32>(conn)?;

    // Convert the permission enum to a string.
    let perm_str = perm.to_string();

    // Check if the permission is already assigned to this role.
    let permission_exists = select(exists(
        role_permission_platform::table
            // Nullable column, so match types:
            .filter(platform_role_id.nullable().eq(Some(platform_role_id_value)))
            .filter(permission.eq(perm_str.as_str()))
    ))
    .get_result(conn)?;

    if permission_exists {
        // Permission already exists; no modification is made.
        return Ok(0);
    }

    // Define a new permission record.
    #[derive(Insertable)]
    #[diesel(table_name = role_permission_platform)]
    struct NewPermission<'a> {
        platform_role_id: Option<i32>,
        permission: &'a str,
    }

    let new_permission = NewPermission {
        platform_role_id: Some(platform_role_id_value),
        permission: perm_str.as_str(),
    };

    // Insert the new permission record into the role_permission_platform table.
    insert_into(role_permission_platform::table)
        .values(&new_permission)
        .execute(conn)
}
