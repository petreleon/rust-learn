use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses_organizations, delegated_permissions, reward_policies, role_permission_organization,
    role_permission_platform, user_role_organization, user_role_platform,
};
use crate::models::delegated_permission::{DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM};
use crate::models::notification::NewNotification;
use crate::models::reward_fraud_block::{
    NewRewardFraudBlock, RewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE,
    REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION, REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
    REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_fraud_block_repository;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::utils::notifications::create_notifications_bulk;

#[derive(Debug, Clone, Deserialize)]
pub struct RewardFraudBlockRequest {
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListRewardFraudBlocksRequest {
    pub scope_type: Option<String>,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListRewardFraudBlocksResponse {
    pub blocks: Vec<RewardFraudBlock>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardFraudBlockAuditEvent {
    pub fraud_block_id: i64,
    pub event_type: String,
    pub actor_user_id: i32,
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RewardFraudBlockError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for RewardFraudBlockError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardFraudBlockError::NotFound,
            other => RewardFraudBlockError::Database(other.to_string()),
        }
    }
}

pub async fn create_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: RewardFraudBlockRequest,
) -> Result<RewardFraudBlock, RewardFraudBlockError> {
    let block = normalize_block_request(request)?;
    ensure_scope_permission(conn, actor_user_id, &block.scope_type).await?;

    let created = reward_fraud_block_repository::create_reward_fraud_block(
        conn,
        NewRewardFraudBlock {
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            created_by_user_id: actor_user_id,
            expires_at: block.expires_at,
        },
    )
    .await
    .map_err(RewardFraudBlockError::from)?;
    notify_reward_fraud_block_transition(conn, &created, "created").await?;
    Ok(created)
}

pub async fn revoke_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    block_id: i64,
) -> Result<RewardFraudBlock, RewardFraudBlockError> {
    let existing = reward_fraud_block_repository::find_reward_fraud_block(conn, block_id).await?;
    ensure_scope_permission(conn, actor_user_id, &existing.scope_type).await?;

    if existing.revoked_at.is_some() {
        return Ok(existing);
    }

    let revoked = reward_fraud_block_repository::revoke_reward_fraud_block(
        conn,
        block_id,
        actor_user_id,
        Utc::now(),
    )
    .await
    .map_err(RewardFraudBlockError::from)?;
    notify_reward_fraud_block_transition(conn, &revoked, "revoked").await?;
    Ok(revoked)
}

pub async fn list_reward_fraud_blocks(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: ListRewardFraudBlocksRequest,
) -> Result<ListRewardFraudBlocksResponse, RewardFraudBlockError> {
    ensure_reward_fraud_report_permission(conn, actor_user_id).await?;

    let scope_type = request
        .scope_type
        .as_deref()
        .map(normalize_scope_type)
        .transpose()?;

    let (blocks, total) = reward_fraud_block_repository::list_reward_fraud_blocks(
        conn,
        reward_fraud_block_repository::RewardFraudBlockFilter {
            scope_type,
            teacher_user_id: request.teacher_user_id,
            organization_id: request.organization_id,
            course_id: request.course_id,
            reward_policy_id: request.reward_policy_id,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        },
    )
    .await
    .map_err(RewardFraudBlockError::from)?;

    let limit = request.limit.unwrap_or(100).clamp(1, 500);
    let offset = request.offset.unwrap_or(0).max(0);

    Ok(ListRewardFraudBlocksResponse {
        blocks,
        limit,
        offset,
        total,
    })
}

pub async fn reward_fraud_block_audit_history(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    block_id: i64,
) -> Result<Vec<RewardFraudBlockAuditEvent>, RewardFraudBlockError> {
    ensure_reward_fraud_report_permission(conn, actor_user_id).await?;
    let block = reward_fraud_block_repository::find_reward_fraud_block(conn, block_id).await?;

    let mut events = vec![RewardFraudBlockAuditEvent {
        fraud_block_id: block.id,
        event_type: "created".to_string(),
        actor_user_id: block.created_by_user_id,
        scope_type: block.scope_type.clone(),
        teacher_user_id: block.teacher_user_id,
        organization_id: block.organization_id,
        course_id: block.course_id,
        reward_policy_id: block.reward_policy_id,
        reason: block.reason.clone(),
        evidence_reference: block.evidence_reference.clone(),
        occurred_at: block.created_at,
    }];

    if let (Some(revoked_by_user_id), Some(revoked_at)) =
        (block.revoked_by_user_id, block.revoked_at)
    {
        events.push(RewardFraudBlockAuditEvent {
            fraud_block_id: block.id,
            event_type: "revoked".to_string(),
            actor_user_id: revoked_by_user_id,
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            occurred_at: revoked_at,
        });
    }

    Ok(events)
}

async fn notify_reward_fraud_block_transition(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlock,
    event_type: &str,
) -> Result<(), RewardFraudBlockError> {
    let recipients = reward_fraud_block_notification_recipients(conn, block).await?;
    if recipients.is_empty() {
        return Ok(());
    }

    let title = format!("reward_fraud_block:{}", event_type);
    let body = format!(
        "Reward fraud block #{} was {} for {} scope. Reason: {}",
        block.id, event_type, block.scope_type, block.reason
    );

    let notifications = recipients
        .into_iter()
        .map(|user_id| NewNotification {
            user_id: Some(user_id),
            title: title.as_str(),
            body: body.as_str(),
        })
        .collect::<Vec<_>>();
    create_notifications_bulk(conn, notifications.as_slice())
        .await
        .map_err(|e| RewardFraudBlockError::Database(e.to_string()))?;

    Ok(())
}

async fn reward_fraud_block_notification_recipients(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlock,
) -> Result<HashSet<i32>, RewardFraudBlockError> {
    let mut recipients = HashSet::new();

    if let Some(teacher_user_id) = block.teacher_user_id {
        recipients.insert(teacher_user_id);
    }

    if let Some(organization_id) = block.organization_id {
        recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
    }

    if let Some(course_id) = block.course_id {
        for organization_id in course_organization_ids(conn, course_id).await? {
            recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
        }
    }

    if let Some(reward_policy_id) = block.reward_policy_id {
        recipients.extend(reward_policy_operator_user_ids(conn, reward_policy_id).await?);
    }

    recipients.extend(platform_reward_reviewer_user_ids(conn).await?);
    Ok(recipients)
}

async fn platform_reward_reviewer_user_ids(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let now = Utc::now();
    let permissions = platform_fraud_notification_permissions();
    let mut reviewers = user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .select(user_role_platform::user_id)
        .filter(role_permission_platform::permission.eq_any(permissions.as_slice()))
        .distinct()
        .load::<Option<i32>>(conn)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    reviewers.extend(
        delegated_permissions::table
            .select(delegated_permissions::grantee_user_id)
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permissions.as_slice()))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            )
            .distinct()
            .load::<i32>(conn)
            .await?,
    );

    reviewers.sort_unstable();
    reviewers.dedup();
    Ok(reviewers)
}

async fn organization_reward_operator_user_ids(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let now = Utc::now();
    let permissions = organization_fraud_notification_permissions();
    let mut operators = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(Some(organization_id)))
        .filter(role_permission_organization::permission.eq_any(permissions.as_slice()))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    operators.extend(
        delegated_permissions::table
            .select(delegated_permissions::grantee_user_id)
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permissions.as_slice()))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            )
            .distinct()
            .load::<i32>(conn)
            .await?,
    );

    operators.sort_unstable();
    operators.dedup();
    Ok(operators)
}

fn platform_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_REWARD_AUDIT.to_string(),
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS.to_string(),
    ]
}

fn organization_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
    ]
}

async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(RewardFraudBlockError::from)
}

async fn reward_policy_operator_user_ids(
    conn: &mut AsyncPgConnection,
    reward_policy_id: i64,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let (organization_id, course_id) = reward_policies::table
        .find(reward_policy_id)
        .select((reward_policies::organization_id, reward_policies::course_id))
        .first::<(Option<i32>, Option<i32>)>(conn)
        .await?;

    let mut recipients = Vec::new();
    if let Some(organization_id) = organization_id {
        recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
    }
    if let Some(course_id) = course_id {
        for organization_id in course_organization_ids(conn, course_id).await? {
            recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
        }
    }
    Ok(recipients)
}

struct NormalizedRewardFraudBlock {
    scope_type: String,
    teacher_user_id: Option<i32>,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    reward_policy_id: Option<i64>,
    reason: String,
    evidence_reference: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

fn normalize_block_request(
    request: RewardFraudBlockRequest,
) -> Result<NormalizedRewardFraudBlock, RewardFraudBlockError> {
    let scope_type = normalize_scope_type(&request.scope_type)?;
    let reason = request.reason.trim().to_string();
    if reason.is_empty() {
        return Err(RewardFraudBlockError::InvalidInput(
            "block reason is required".to_string(),
        ));
    }

    let evidence_reference = request
        .evidence_reference
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let target_count = [
        request.teacher_user_id.is_some(),
        request.organization_id.is_some(),
        request.course_id.is_some(),
        request.reward_policy_id.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if target_count != 1 {
        return Err(RewardFraudBlockError::InvalidInput(
            "exactly one fraud block target is required".to_string(),
        ));
    }

    match scope_type.as_str() {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER if request.teacher_user_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION if request.organization_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_COURSE if request.course_id.is_some() => {}
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY if request.reward_policy_id.is_some() => {}
        _ => {
            return Err(RewardFraudBlockError::InvalidInput(
                "fraud block target does not match scope type".to_string(),
            ));
        }
    }

    Ok(NormalizedRewardFraudBlock {
        scope_type,
        teacher_user_id: request.teacher_user_id,
        organization_id: request.organization_id,
        course_id: request.course_id,
        reward_policy_id: request.reward_policy_id,
        reason,
        evidence_reference,
        expires_at: request.expires_at,
    })
}

fn normalize_scope_type(scope_type: &str) -> Result<String, RewardFraudBlockError> {
    match scope_type.trim() {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => {
            Ok(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string())
        }
        REWARD_FRAUD_BLOCK_SCOPE_COURSE => Ok(REWARD_FRAUD_BLOCK_SCOPE_COURSE.to_string()),
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY.to_string())
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}

async fn ensure_scope_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    scope_type: &str,
) -> Result<(), RewardFraudBlockError> {
    let permissions = required_permissions_for_scope(scope_type)?;
    for permission in permissions.iter() {
        if user_permission_platform_request(conn, actor_user_id, &permission.to_string()).await? {
            return Ok(());
        }
    }

    Err(RewardFraudBlockError::PermissionDenied(
        permissions
            .into_iter()
            .map(|permission| permission.to_string())
            .collect::<Vec<_>>()
            .join(" or "),
    ))
}

async fn ensure_reward_fraud_report_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<(), RewardFraudBlockError> {
    for permission in [
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
    ] {
        if user_permission_platform_request(conn, actor_user_id, &permission.to_string()).await? {
            return Ok(());
        }
    }

    Err(RewardFraudBlockError::PermissionDenied(format!(
        "{} or {}",
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS
    )))
}

fn required_permissions_for_scope(
    scope_type: &str,
) -> Result<Vec<Permissions>, RewardFraudBlockError> {
    match scope_type {
        REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(vec![
            Permissions::BLOCK_REWARD_TEACHER,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => Ok(vec![
            Permissions::BLOCK_REWARD_ORGANIZATION,
            Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
        ]),
        REWARD_FRAUD_BLOCK_SCOPE_COURSE | REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => {
            Ok(vec![Permissions::MANAGE_REWARD_FRAUD_BLOCKS])
        }
        _ => Err(RewardFraudBlockError::InvalidInput(
            "unsupported fraud block scope type".to_string(),
        )),
    }
}
