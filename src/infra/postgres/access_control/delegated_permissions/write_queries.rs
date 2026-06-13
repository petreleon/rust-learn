use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionCreate, DelegatedPermissionError, DelegatedPermissionOutput,
};
use crate::db::schema::delegated_permissions;
use crate::infra::postgres::access_control::delegated_permissions::mappers::map_error;
use crate::infra::postgres::access_control::delegated_permissions::read_queries;
use crate::models::delegated_permission::{DelegatedPermission, NewDelegatedPermission};

pub(super) async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation: DelegatedPermissionCreate,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    let new_delegation = NewDelegatedPermission {
        course_id: delegation.course_id,
        expires_at: delegation.expires_at,
        grantee_user_id: delegation.grantee_user_id,
        grantor_user_id: delegation.grantor_user_id,
        organization_id: delegation.organization_id,
        permission: delegation.permission,
        reason: delegation.reason,
        scope_type: delegation.scope_type,
    };

    if let Some(inserted) = diesel::insert_into(delegated_permissions::table)
        .values(&new_delegation)
        .on_conflict_do_nothing()
        .get_result::<DelegatedPermission>(conn)
        .await
        .optional()
        .map_err(map_error)?
    {
        return Ok(inserted.into());
    }

    read_queries::find_active_delegated_permission(
        conn,
        new_delegation.grantee_user_id,
        &new_delegation.permission,
        &new_delegation.scope_type,
        new_delegation.organization_id,
        new_delegation.course_id,
    )
    .await?
    .ok_or(DelegatedPermissionError::NotFound)
}

pub(super) async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
    revoked_by_user_id: i32,
    revoke_reason: Option<String>,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
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
    .get_result::<DelegatedPermission>(conn)
    .await
    .map(Into::into)
    .map_err(map_error)
}
