// src/utils/db_utils/platform_permission_utils.rs

use diesel::prelude::*;
use diesel::insert_into;
use diesel::QueryResult;
use diesel::dsl::{exists, select};
use crate::db::schema::role_permission_platform;
use crate::db::schema::role_permission_platform::dsl::*;
use crate::db::schema::roles::dsl::{roles, name, id as role_id_column};
use crate::config::constants::roles::Roles;

/// Assigns a permission to a role on the platform after checking if it already exists.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `role` - A role variant from the `Roles` enum.
/// * `permission_str` - The permission string to be assigned to the role.
///
/// # Returns
///
/// A `QueryResult<usize>` indicating the number of rows affected.  
/// Returns 0 if the permission is already assigned.
pub fn assign_permission_to_role_platform(
    conn: &mut PgConnection,
    role: Roles,
    permission_str: &str,
) -> QueryResult<usize> {
    // Retrieve the role id based on the role name.
    let role_id_value = roles
        .filter(name.eq(role.to_string()))
        .select(role_id_column)
        .first::<i32>(conn)?;

    // Check if the permission is already assigned to this role.
    let permission_exists = select(exists(
        role_permission_platform::table
            .filter(role_id.eq(role_id_value))
            .filter(permission.eq(permission_str))
    ))
    .get_result(conn)?;

    if permission_exists {
        // Permission already exists; no modification is made.
        return Ok(0);
    }

    // Define a new permission record.
    #[derive(Insertable)]
    #[table_name = "role_permission_platform"]
    struct NewPermission<'a> {
        role_id: i32,
        permission: &'a str,
    }

    let new_permission = NewPermission {
        role_id: role_id_value,
        permission: permission_str,
    };

    // Insert the new permission record into the role_permission_platform table.
    insert_into(role_permission_platform::table)
        .values(&new_permission)
        .execute(conn)
}
