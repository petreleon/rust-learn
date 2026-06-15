mod mappers;
mod read_queries;
pub(crate) mod records;
pub mod store;
pub mod use_case;
mod write_queries;

use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

pub use crate::application::access_control::manage_delegated_permissions::DelegatedPermissionFilter;
use crate::infra::postgres::access_control::permission_delegations;
use crate::infra::postgres::models::delegated_permission::{
    DelegatedPermission, NewDelegatedPermission,
};

pub async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    new_delegation: NewDelegatedPermission,
) -> QueryResult<DelegatedPermission> {
    records::create_delegated_permission(conn, new_delegation).await
}

pub async fn find_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
) -> QueryResult<DelegatedPermission> {
    records::find_delegated_permission(conn, delegation_id).await
}

pub async fn find_active_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> QueryResult<Option<DelegatedPermission>> {
    records::find_active_delegated_permission(
        conn,
        grantee_user_id,
        permission,
        scope_type,
        organization_id,
        course_id,
    )
    .await
}

pub async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionFilter,
) -> QueryResult<Vec<DelegatedPermission>> {
    records::list_delegated_permissions(conn, filter.into()).await
}

pub async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
    revoked_by_user_id: i32,
    revoke_reason: Option<String>,
) -> QueryResult<DelegatedPermission> {
    records::revoke_delegated_permission(conn, delegation_id, revoked_by_user_id, revoke_reason)
        .await
}

pub async fn has_active_platform_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_delegations::has_active_platform_delegation(conn, grantee_user_id, permission).await
}

pub async fn has_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_delegations::has_active_organization_delegation(
        conn,
        grantee_user_id,
        organization_id,
        permission,
    )
    .await
}

pub async fn has_active_course_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    permission_delegations::has_active_course_delegation(
        conn,
        grantee_user_id,
        course_id,
        permission,
    )
    .await
}
