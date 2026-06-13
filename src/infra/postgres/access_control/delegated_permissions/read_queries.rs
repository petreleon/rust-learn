use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionFilter, DelegatedPermissionOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, delegated_permissions, organizations};
use crate::infra::postgres::access_control::delegated_permissions::mappers::map_error;
use crate::models::delegated_permission::DelegatedPermission;
use crate::repositories::platform_repository::user_permission_platform_request;

pub(super) async fn can_delegate_reward_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    user_permission_platform_request(
        conn,
        user_id,
        &Permissions::DELEGATE_REWARD_APPROVAL.to_string(),
    )
    .await
    .map_err(map_error)
}

pub(super) async fn organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    select(exists(
        organizations::table.filter(organizations::id.eq(organization_id)),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(map_error)
}

pub(super) async fn course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, DelegatedPermissionError> {
    select(exists(courses::table.filter(courses::id.eq(course_id))))
        .get_result::<bool>(conn)
        .await
        .map_err(map_error)
}

pub(super) async fn find_active_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantee_user_id: i32,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<Option<DelegatedPermissionOutput>, DelegatedPermissionError> {
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
        Some(id) => query.filter(delegated_permissions::organization_id.eq(Some(id))),
        None => query.filter(delegated_permissions::organization_id.is_null()),
    };
    query = match course_id {
        Some(id) => query.filter(delegated_permissions::course_id.eq(Some(id))),
        None => query.filter(delegated_permissions::course_id.is_null()),
    };

    query
        .order(delegated_permissions::created_at.desc())
        .first::<DelegatedPermission>(conn)
        .await
        .optional()
        .map(|item| item.map(Into::into))
        .map_err(map_error)
}

pub(super) async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    filter: DelegatedPermissionFilter,
) -> Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError> {
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
        .load::<DelegatedPermission>(conn)
        .await
        .map(|items| items.into_iter().map(Into::into).collect())
        .map_err(map_error)
}
