use crate::config::constants::permissions::Permissions;
use crate::models::reward_fraud_block::{
    NewRewardFraudBlock, RewardFraudBlock, REWARD_FRAUD_BLOCK_SCOPE_COURSE,
    REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION, REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
    REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_fraud_block_repository;
use chrono::{DateTime, Utc};
use diesel_async::AsyncPgConnection;
use serde::Deserialize;

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

    reward_fraud_block_repository::create_reward_fraud_block(
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
    .map_err(RewardFraudBlockError::from)
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

    reward_fraud_block_repository::revoke_reward_fraud_block(
        conn,
        block_id,
        actor_user_id,
        Utc::now(),
    )
    .await
    .map_err(RewardFraudBlockError::from)
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
