use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::delegated_permissions;
use crate::infra::postgres::models::delegated_permission::{
    DelegatedPermission, NewDelegatedPermission,
};

#[derive(Debug, Default)]
pub(crate) struct DelegatedPermissionRecordFilter {
    pub(crate) grantor_user_id: Option<i32>,
    pub(crate) grantee_user_id: Option<i32>,
    pub(crate) permission: Option<String>,
    pub(crate) scope_type: Option<String>,
    pub(crate) organization_id: Option<i32>,
    pub(crate) course_id: Option<i32>,
    pub(crate) active: Option<bool>,
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
}

pub(crate) async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    new_delegation: NewDelegatedPermission,
) -> QueryResult<DelegatedPermission> {
    if let Some(inserted) = diesel::insert_into(delegated_permissions::table)
        .values(&new_delegation)
        .on_conflict_do_nothing()
        .get_result(conn)
        .await
        .optional()?
    {
        return Ok(inserted);
    }

    find_active_delegated_permission(
        conn,
        new_delegation.grantee_user_id,
        &new_delegation.permission,
        &new_delegation.scope_type,
        new_delegation.organization_id,
        new_delegation.course_id,
    )
    .await?
    .ok_or(diesel::result::Error::NotFound)
}

pub(crate) async fn find_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
) -> QueryResult<DelegatedPermission> {
    delegated_permissions::table
        .find(delegation_id)
        .first(conn)
        .await
}

pub(crate) async fn find_active_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> QueryResult<Option<DelegatedPermission>> {
    let now = Utc::now();
    let mut query = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(grantee_user_id))
        .filter(delegated_permissions::permission.eq(permission))
        .filter(delegated_permissions::scope_type.eq(scope_type))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .into_boxed();

    query = match organization_id {
        Some(organization_id) => {
            query.filter(delegated_permissions::organization_id.eq(Some(organization_id)))
        }
        None => query.filter(delegated_permissions::organization_id.is_null()),
    };
    query = match course_id {
        Some(course_id) => query.filter(delegated_permissions::course_id.eq(Some(course_id))),
        None => query.filter(delegated_permissions::course_id.is_null()),
    };

    query
        .order(delegated_permissions::created_at.desc())
        .first(conn)
        .await
        .optional()
}

pub(crate) async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionRecordFilter,
) -> QueryResult<Vec<DelegatedPermission>> {
    let now = Utc::now();
    let mut query = delegated_permissions::table.into_boxed();

    if let Some(id) = filter.grantor_user_id {
        query = query.filter(delegated_permissions::grantor_user_id.eq(id));
    }
    if let Some(id) = filter.grantee_user_id {
        query = query.filter(delegated_permissions::grantee_user_id.eq(id));
    }
    if let Some(permission) = filter.permission {
        query = query.filter(delegated_permissions::permission.eq(permission));
    }
    if let Some(scope_type) = filter.scope_type {
        query = query.filter(delegated_permissions::scope_type.eq(scope_type));
    }
    if let Some(id) = filter.organization_id {
        query = query.filter(delegated_permissions::organization_id.eq(Some(id)));
    }
    if let Some(id) = filter.course_id {
        query = query.filter(delegated_permissions::course_id.eq(Some(id)));
    }
    if let Some(active) = filter.active {
        query = if active {
            query
                .filter(delegated_permissions::revoked_at.is_null())
                .filter(
                    delegated_permissions::expires_at
                        .is_null()
                        .or(delegated_permissions::expires_at.gt(now)),
                )
        } else {
            query.filter(
                delegated_permissions::revoked_at
                    .is_not_null()
                    .or(delegated_permissions::expires_at.le(now)),
            )
        };
    }

    query
        .order(delegated_permissions::created_at.desc())
        .limit(filter.limit.unwrap_or(100).clamp(1, 500))
        .offset(filter.offset.unwrap_or(0).max(0))
        .load(conn)
        .await
}

pub(crate) async fn revoke_delegated_permission(
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
