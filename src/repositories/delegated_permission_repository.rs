use crate::db::schema::delegated_permissions;
use crate::models::delegated_permission::{DelegatedPermission, NewDelegatedPermission};
use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug, Default)]
pub struct DelegatedPermissionFilter {
    pub grantor_user_id: Option<i32>,
    pub grantee_user_id: Option<i32>,
    pub permission: Option<String>,
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_delegated_permission(
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

pub async fn find_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
) -> QueryResult<DelegatedPermission> {
    delegated_permissions::table
        .find(delegation_id)
        .first(conn)
        .await
}

pub async fn find_active_delegated_permission(
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
            query.filter(delegated_permissions::organization_id.eq(organization_id))
        }
        None => query.filter(delegated_permissions::organization_id.is_null()),
    };
    query = match course_id {
        Some(course_id) => query.filter(delegated_permissions::course_id.eq(course_id)),
        None => query.filter(delegated_permissions::course_id.is_null()),
    };

    query
        .order(delegated_permissions::created_at.desc())
        .first(conn)
        .await
        .optional()
}

pub async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionFilter,
) -> QueryResult<Vec<DelegatedPermission>> {
    let now = Utc::now();
    let mut query = delegated_permissions::table.into_boxed();

    if let Some(grantor_user_id) = filter.grantor_user_id {
        query = query.filter(delegated_permissions::grantor_user_id.eq(grantor_user_id));
    }
    if let Some(grantee_user_id) = filter.grantee_user_id {
        query = query.filter(delegated_permissions::grantee_user_id.eq(grantee_user_id));
    }
    if let Some(permission) = filter.permission {
        query = query.filter(delegated_permissions::permission.eq(permission));
    }
    if let Some(scope_type) = filter.scope_type {
        query = query.filter(delegated_permissions::scope_type.eq(scope_type));
    }
    if let Some(organization_id) = filter.organization_id {
        query = query.filter(delegated_permissions::organization_id.eq(Some(organization_id)));
    }
    if let Some(course_id) = filter.course_id {
        query = query.filter(delegated_permissions::course_id.eq(Some(course_id)));
    }
    if let Some(active) = filter.active {
        if active {
            query = query
                .filter(delegated_permissions::revoked_at.is_null())
                .filter(
                    delegated_permissions::expires_at
                        .is_null()
                        .or(delegated_permissions::expires_at.gt(now)),
                );
        } else {
            query = query.filter(
                delegated_permissions::revoked_at
                    .is_not_null()
                    .or(delegated_permissions::expires_at.le(now)),
            );
        }
    }

    query
        .order(delegated_permissions::created_at.desc())
        .limit(filter.limit.unwrap_or(100).clamp(1, 500))
        .offset(filter.offset.unwrap_or(0).max(0))
        .load(conn)
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
