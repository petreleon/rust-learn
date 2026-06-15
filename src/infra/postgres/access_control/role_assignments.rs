use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::config::constants::permissions::Permissions;
use crate::config::constants::roles::Roles;
use crate::infra::postgres::access_control::{
    hierarchy_records, organization_role_records, permission_assignment_records,
    platform_role_records, role_catalog_store,
};

pub async fn assign_platform_role_to_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role: Roles,
) -> QueryResult<usize> {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id).await
}

pub async fn assign_platform_permission_to_role(
    conn: &mut AsyncPgConnection,
    role: Roles,
    permission: Permissions,
) -> QueryResult<usize> {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, &role.to_string()).await?;
    permission_assignment_records::assign_platform_permission_to_role(
        conn,
        role_id,
        &permission.to_string(),
    )
    .await
}

pub async fn assign_organization_role_with_hierarchy(
    conn: &mut AsyncPgConnection,
    assigner_id: i32,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) -> QueryResult<usize> {
    let assigner_level =
        hierarchy_records::organization_min_level_for_user(conn, assigner_id, organization_id)
            .await?
            .ok_or(diesel::result::Error::NotFound)?;
    let assignee_level =
        hierarchy_records::organization_min_level_for_user(conn, user_id, organization_id).await?;
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name).await?;
    let role_level = hierarchy_records::organization_role_level(conn, role_id).await?;

    if assigner_level >= role_level || assignee_level.is_some_and(|level| assigner_level >= level) {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
}
