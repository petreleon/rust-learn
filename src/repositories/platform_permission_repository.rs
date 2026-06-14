use crate::config::constants::permissions::Permissions;
use crate::config::constants::roles::Roles;
use crate::infra::postgres::access_control::permission_assignment_records;
use crate::infra::postgres::access_control::role_catalog_store;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

/// Compatibility bridge for assigning a permission to a platform role.
pub async fn assign_permission_to_role_platform(
    conn: &mut AsyncPgConnection,
    role: Roles,
    perm: Permissions,
) -> QueryResult<usize> {
    // Retrieve the platform_role_id based on the role name.
    let platform_role_id_value =
        role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;

    // Convert the permission enum to a string.
    let perm_str = perm.to_string();

    // Assign permission to role
    permission_assignment_records::assign_platform_permission_to_role(
        conn,
        platform_role_id_value,
        &perm_str,
    )
    .await
}
