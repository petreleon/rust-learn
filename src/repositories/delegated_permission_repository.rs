use crate::db::schema::delegated_permissions;
use crate::models::delegated_permission::{DelegatedPermission, NewDelegatedPermission};
use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    new_delegation: NewDelegatedPermission,
) -> QueryResult<DelegatedPermission> {
    diesel::insert_into(delegated_permissions::table)
        .values(&new_delegation)
        .get_result(conn)
        .await
}

pub async fn find_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
) -> QueryResult<DelegatedPermission> {
    delegated_permissions::table
        .find(delegation_id)
        .first(conn)
        .await
}

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
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(grantee_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq("platform"))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result(conn)
    .await
}

pub async fn has_active_organization_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(grantee_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq("organization"))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result(conn)
    .await
}

pub async fn has_active_course_delegation(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(grantee_user_id))
            .filter(delegated_permissions::permission.eq(permission))
            .filter(delegated_permissions::scope_type.eq("course"))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.eq(Some(course_id)))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result(conn)
    .await
}
