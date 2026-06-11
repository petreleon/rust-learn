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
