use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, ListRewardFraudBlocksOutput, ListRewardFraudBlocksQuery,
    RewardFraudBlockAuditEventOutput, RewardFraudBlockOutput,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRewardFraudBlockRequest {
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
    pub blocks: Vec<RewardFraudBlockResponse>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardFraudBlockResponse {
    pub id: i64,
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub created_by_user_id: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<i32>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardFraudBlockAuditEventResponse {
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

impl From<CreateRewardFraudBlockRequest> for CreateRewardFraudBlockCommand {
    fn from(request: CreateRewardFraudBlockRequest) -> Self {
        Self {
            scope_type: request.scope_type,
            teacher_user_id: request.teacher_user_id,
            organization_id: request.organization_id,
            course_id: request.course_id,
            reward_policy_id: request.reward_policy_id,
            reason: request.reason,
            evidence_reference: request.evidence_reference,
            expires_at: request.expires_at,
        }
    }
}

impl From<ListRewardFraudBlocksRequest> for ListRewardFraudBlocksQuery {
    fn from(request: ListRewardFraudBlocksRequest) -> Self {
        Self {
            scope_type: request.scope_type,
            teacher_user_id: request.teacher_user_id,
            organization_id: request.organization_id,
            course_id: request.course_id,
            reward_policy_id: request.reward_policy_id,
            active: request.active,
            limit: request.limit,
            offset: request.offset,
        }
    }
}

impl From<ListRewardFraudBlocksOutput> for ListRewardFraudBlocksResponse {
    fn from(output: ListRewardFraudBlocksOutput) -> Self {
        Self {
            blocks: output
                .blocks
                .into_iter()
                .map(RewardFraudBlockResponse::from)
                .collect(),
            limit: output.limit,
            offset: output.offset,
            total: output.total,
        }
    }
}

impl From<RewardFraudBlockOutput> for RewardFraudBlockResponse {
    fn from(block: RewardFraudBlockOutput) -> Self {
        Self {
            id: block.id,
            scope_type: block.scope_type,
            teacher_user_id: block.teacher_user_id,
            organization_id: block.organization_id,
            course_id: block.course_id,
            reward_policy_id: block.reward_policy_id,
            reason: block.reason,
            evidence_reference: block.evidence_reference,
            created_by_user_id: block.created_by_user_id,
            expires_at: block.expires_at,
            revoked_by_user_id: block.revoked_by_user_id,
            revoked_at: block.revoked_at,
            created_at: block.created_at,
            updated_at: block.updated_at,
        }
    }
}

impl From<RewardFraudBlockAuditEventOutput> for RewardFraudBlockAuditEventResponse {
    fn from(event: RewardFraudBlockAuditEventOutput) -> Self {
        Self {
            fraud_block_id: event.fraud_block_id,
            event_type: event.event_type,
            actor_user_id: event.actor_user_id,
            scope_type: event.scope_type,
            teacher_user_id: event.teacher_user_id,
            organization_id: event.organization_id,
            course_id: event.course_id,
            reward_policy_id: event.reward_policy_id,
            reason: event.reason,
            evidence_reference: event.evidence_reference,
            occurred_at: event.occurred_at,
        }
    }
}
