use diesel::prelude::*;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use crate::models::role_permission_platform::RolePermissionPlatform;
use crate::models::role::PlatformRole;
use crate::config::constants::roles::Roles;
use crate::config::constants::permissions::Permissions;

/// Assigns a permission to a platform role after checking if it already exists.
pub async fn assign_permission_to_role_platform(
    conn: &mut AsyncPgConnection,
    role: Roles,
    perm: Permissions,
) -> QueryResult<usize> {
    // Retrieve the platform_role_id based on the role name.
    let platform_role_id_value = PlatformRole::find_by_name(&role.to_string(), conn).await?;

    // Convert the permission enum to a string.
    let perm_str = perm.to_string();

    // Assign permission to role
    RolePermissionPlatform::assign(conn, platform_role_id_value, &perm_str).await
}
