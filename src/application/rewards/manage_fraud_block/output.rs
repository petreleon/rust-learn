use chrono::{DateTime, Utc};

use crate::domain::rewards::fraud_block::{RewardFraudBlockAuditEventType, RewardFraudBlockScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardFraudBlockOutput {
    pub id: i64,
    pub scope_type: RewardFraudBlockScope,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListRewardFraudBlocksOutput {
    pub blocks: Vec<RewardFraudBlockOutput>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardFraudBlockAuditEventOutput {
    pub fraud_block_id: i64,
    pub event_type: RewardFraudBlockAuditEventType,
    pub actor_user_id: i32,
    pub scope_type: RewardFraudBlockScope,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub occurred_at: DateTime<Utc>,
}
