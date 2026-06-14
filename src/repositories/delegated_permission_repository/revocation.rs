use crate::db::schema::delegated_permissions;
use crate::models::delegated_permission::DelegatedPermission;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
    revoked_by_user_id: i32,
    revoke_reason: Option<String>,
) -> QueryResult<DelegatedPermission> {
    let now = Utc::now();
    diesel::update(
        delegated_permissions::table
            .find(delegation_id)
            .filter(delegated_permissions::revoked_at.is_null()),
    )
    .set((
        delegated_permissions::revoked_at.eq(now),
        delegated_permissions::revoked_by_user_id.eq(Some(revoked_by_user_id)),
        delegated_permissions::revoke_reason.eq(revoke_reason),
        delegated_permissions::updated_at.eq(now),
    ))
    .get_result(conn)
    .await
}

pub async fn has_active_platform_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    crate::infra::postgres::access_control::permission_delegations::has_active_platform_delegation(
        conn,
        grantee_user_id,
        permission,
    )
    .await
}

pub async fn has_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    crate::infra::postgres::access_control::permission_delegations::has_active_organization_delegation(
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
    crate::infra::postgres::access_control::permission_delegations::has_active_course_delegation(
        conn,
        grantee_user_id,
        course_id,
        permission,
    )
    .await
}
