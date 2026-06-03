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

pub async fn list_delegated_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListDelegatedPermissionsRequest,
) -> Result<Vec<DelegatedPermission>, DelegatedPermissionError> {
    ensure_can_delegate_reward_permissions(conn, actor_user_id).await?;

    let permission = request
        .permission
        .as_deref()
        .map(normalize_permission)
        .transpose()?;
    let scope_type = request
        .scope_type
        .as_deref()
        .map(normalize_scope_type)
        .transpose()?;

    delegated_permission_repository::list_delegated_permissions(
        conn,
        delegated_permission_repository::DelegatedPermissionFilter {
            grantor_user_id: request.grantor_user_id,
            grantee_user_id: request.grantee_user_id,
            permission,
            scope_type,
            organization_id: request.organization_id,
            course_id: request.course_id,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(DelegatedPermissionError::from)
}

async fn ensure_can_delegate_reward_permissions(
    conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let permission = Permissions::DELEGATE_REWARD_APPROVAL.to_string();
    if user_permission_platform_request(conn, grantor_user_id, &permission).await? {
        Ok(())
    } else {
        Err(DelegatedPermissionError::PermissionDenied(permission))
    }
}

fn normalize_permission(permission: &str) -> Result<String, DelegatedPermissionError> {
    let permission = permission.trim();
    let parsed = Permissions::from_str(permission).map_err(|_| {
        DelegatedPermissionError::InvalidInput("unsupported delegated permission".to_string())
    })?;

    Ok(parsed.to_string())
}

fn normalize_scope_type(scope_type: &str) -> Result<String, DelegatedPermissionError> {
    let normalized = scope_type.trim().to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        DELEGATED_SCOPE_PLATFORM | DELEGATED_SCOPE_ORGANIZATION | DELEGATED_SCOPE_COURSE => {
            Ok(normalized)
        }
        _ => Err(DelegatedPermissionError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}

fn ensure_permission_can_be_delegated_for_reward_work(
    permission: &str,
) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "APPROVE_REWARD_AMOUNT"
            | "EXECUTE_REWARD_PAYOUT"
            | "VIEW_REWARD_AUDIT"
            | "MANAGE_REWARD_FRAUD_BLOCKS"
            | "BLOCK_REWARD_TEACHER"
            | "BLOCK_REWARD_ORGANIZATION"
            | "SUBMIT_ORG_COURSE_REWARD_EVENT"
            | "VIEW_ORG_REWARD_REPORTS"
            | "MANAGE_ORG_REWARD_BUDGET"
            | "SUBMIT_COURSE_REWARD_EVENT"
            | "CREATE_REWARDABLE_COURSE_EVENT"
            | "APPROVE_STUDENT_REWARD_CANDIDATE"
            | "VIEW_COURSE_REWARD_STATUS"
            | "GRADE_REWARDABLE_ASSESSMENT"
            | "MANAGE_COURSE_REWARD_RULES"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission is not delegatable for reward work".to_string(),
        ))
    }
}

async fn normalize_scope_ids(
    conn: &mut AsyncPgConnection,
    permission: &str,
    scope_type: &str,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(Option<i32>, Option<i32>), DelegatedPermissionError> {
    match scope_type {
        DELEGATED_SCOPE_PLATFORM => {
            ensure_platform_permission_scope(permission)?;
            if organization_id.is_some() || course_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "platform delegation cannot include organization_id or course_id".to_string(),
                ));
            }
            Ok((None, None))
        }
        DELEGATED_SCOPE_ORGANIZATION => {
            ensure_organization_permission_scope(permission)?;
            let organization_id = organization_id.ok_or_else(|| {
                DelegatedPermissionError::InvalidInput(
                    "organization delegation requires organization_id".to_string(),
                )
            })?;
            if course_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "organization delegation cannot include course_id".to_string(),
                ));
            }
            ensure_organization_exists(conn, organization_id).await?;
            Ok((Some(organization_id), None))
        }
        DELEGATED_SCOPE_COURSE => {
            ensure_course_permission_scope(permission)?;
            let course_id = course_id.ok_or_else(|| {
                DelegatedPermissionError::InvalidInput(
                    "course delegation requires course_id".to_string(),
                )
            })?;
            if organization_id.is_some() {
                return Err(DelegatedPermissionError::InvalidInput(
                    "course delegation cannot include organization_id".to_string(),
                ));
            }
            ensure_course_exists(conn, course_id).await?;
            Ok((None, Some(course_id)))
        }
        _ => Err(DelegatedPermissionError::InvalidInput(
            "unsupported delegated permission scope".to_string(),
        )),
    }
}

fn ensure_platform_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "APPROVE_REWARD_AMOUNT"
            | "EXECUTE_REWARD_PAYOUT"
            | "VIEW_REWARD_AUDIT"
            | "MANAGE_REWARD_FRAUD_BLOCKS"
            | "BLOCK_REWARD_TEACHER"
            | "BLOCK_REWARD_ORGANIZATION"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at platform scope".to_string(),
        ))
    }
}

fn ensure_organization_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "SUBMIT_ORG_COURSE_REWARD_EVENT" | "VIEW_ORG_REWARD_REPORTS" | "MANAGE_ORG_REWARD_BUDGET"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at organization scope".to_string(),
        ))
    }
}

fn ensure_course_permission_scope(permission: &str) -> Result<(), DelegatedPermissionError> {
    if matches!(
        permission,
        "SUBMIT_COURSE_REWARD_EVENT"
            | "CREATE_REWARDABLE_COURSE_EVENT"
            | "APPROVE_STUDENT_REWARD_CANDIDATE"
            | "VIEW_COURSE_REWARD_STATUS"
            | "GRADE_REWARDABLE_ASSESSMENT"
            | "MANAGE_COURSE_REWARD_RULES"
    ) {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "permission cannot be delegated at course scope".to_string(),
        ))
    }
}

async fn ensure_organization_exists(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let exists = select(exists(
        organizations::table.filter(organizations::id.eq(organization_id)),
    ))
    .get_result::<bool>(conn)
    .await?;

    if exists {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "organization scope does not exist".to_string(),
        ))
    }
}

async fn ensure_course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let exists = select(exists(courses::table.filter(courses::id.eq(course_id))))
        .get_result::<bool>(conn)
        .await?;

    if exists {
        Ok(())
    } else {
        Err(DelegatedPermissionError::InvalidInput(
            "course scope does not exist".to_string(),
        ))
    }
}
