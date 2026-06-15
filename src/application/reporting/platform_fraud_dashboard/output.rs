use chrono::{DateTime, Utc};

use crate::domain::rewards::fraud_block::RewardFraudBlockScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformFraudDashboardOutput {
    pub active_total: i64,
    pub active_by_scope: FraudBlockScopeSummaryOutput,
    pub active_blocks: Vec<FraudBlockDashboardRowOutput>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FraudBlockScopeSummaryOutput {
    pub teacher: i64,
    pub organization: i64,
    pub course: i64,
    pub reward_policy: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FraudBlockDashboardRowOutput {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
