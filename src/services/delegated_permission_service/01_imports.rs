use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, organizations};
use crate::models::delegated_permission::{
    DelegatedPermission, GrantDelegatedPermissionRequest, NewDelegatedPermission,
    DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
use crate::repositories::delegated_permission_repository;
use crate::repositories::platform_repository::user_permission_platform_request;
use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum DelegatedPermissionError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Database(String),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListDelegatedPermissionsRequest {
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

impl From<diesel::result::Error> for DelegatedPermissionError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => DelegatedPermissionError::NotFound,
            other => DelegatedPermissionError::Database(other.to_string()),
        }
    }
}

pub async fn grant_delegated_permission(
    conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
    request: GrantDelegatedPermissionRequest,
) -> Result<DelegatedPermission, DelegatedPermissionError> {
    ensure_can_delegate_reward_permissions(conn, grantor_user_id).await?;

    let permission = normalize_permission(&request.permission)?;
    ensure_permission_can_be_delegated_for_reward_work(&permission)?;

    let scope_type = normalize_scope_type(&request.scope_type)?;
    let (organization_id, course_id) = normalize_scope_ids(
        conn,
        &permission,
        &scope_type,
        request.organization_id,
        request.course_id,
    )
    .await?;

    if let Some(expires_at) = request.expires_at {
        if expires_at <= Utc::now() {
            return Err(DelegatedPermissionError::InvalidInput(
                "delegated permission expiration must be in the future".to_string(),
            ));
        }
    }

    if let Some(existing) = delegated_permission_repository::find_active_delegated_permission(
        conn,
        request.grantee_user_id,
        &permission,
        &scope_type,
        organization_id,
        course_id,
    )
    .await?
    {
        log::info!(
            "event=delegated_permission_idempotent_replay delegation_id={} grantor_user_id={} grantee_user_id={} permission={} scope_type={}",
            existing.id,
            grantor_user_id,
            existing.grantee_user_id,
            existing.permission,
            existing.scope_type
        );
        return Ok(existing);
    }

    let delegation = delegated_permission_repository::create_delegated_permission(
        conn,
        NewDelegatedPermission {
            grantor_user_id,
            grantee_user_id: request.grantee_user_id,
            permission,
            scope_type,
            organization_id,
            course_id,
            reason: request.reason,
            expires_at: request.expires_at,
        },
    )
    .await?;

    log::info!(
        "event=delegated_permission_granted delegation_id={} grantor_user_id={} grantee_user_id={} permission={} scope_type={}",
        delegation.id,
        delegation.grantor_user_id,
        delegation.grantee_user_id,
        delegation.permission,
        delegation.scope_type
    );

    Ok(delegation)
}

pub async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    delegation_id: i64,
    revoke_reason: Option<String>,
) -> Result<DelegatedPermission, DelegatedPermissionError> {
    ensure_can_delegate_reward_permissions(conn, actor_user_id).await?;

    let delegation = delegated_permission_repository::revoke_delegated_permission(
        conn,
        delegation_id,
        actor_user_id,
        revoke_reason,
    )
    .await?;

    log::info!(
        "event=delegated_permission_revoked delegation_id={} revoked_by_user_id={}",
        delegation.id,
        actor_user_id
    );

    Ok(delegation)
}
