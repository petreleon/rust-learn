use crate::infra::postgres::access_control::delegated_permissions::records as delegated_permission_records;
use crate::infra::postgres::access_control::permission_delegations;
use crate::models::delegated_permission::DelegatedPermission;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

pub async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
    revoked_by_user_id: i32,
    revoke_reason: Option<String>,
) -> QueryResult<DelegatedPermission> {
    delegated_permission_records::revoke_delegated_permission(
        conn,
        delegation_id,
        revoked_by_user_id,
        revoke_reason,
    )
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
