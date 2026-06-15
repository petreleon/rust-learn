use chrono::{DateTime, Utc};

use crate::domain::rewards::fraud_block::RewardFraudBlockScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRewardFraudBlockCommand {
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardFraudBlockDraft {
    pub created_by_user_id: i32,
    pub scope_type: RewardFraudBlockScope,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
