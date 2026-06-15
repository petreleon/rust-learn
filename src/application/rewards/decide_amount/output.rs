use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardAmountDecisionOutput {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: RewardCandidateSourceScope,
    pub source_organization_id: Option<i32>,
    pub event_type: RewardEventType,
    pub idempotency_key: String,
    pub evidence: JsonValue,
    pub status: RewardCandidateStatus,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: Option<String>,
    pub teacher_decided_at: Option<DateTime<Utc>>,
    pub amount_reviewer_user_id: Option<i32>,
    pub approved_amount: Option<BigDecimal>,
    pub amount_decision_reason: Option<String>,
    pub amount_decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
