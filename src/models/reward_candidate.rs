use crate::db::schema::reward_candidates;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

pub const REWARD_EVENT_ASSESSMENT_COMPLETION: &str = "assessment_completion";
pub const REWARD_EVENT_COURSE_COMPLETION: &str = "course_completion";
pub const REWARD_EVENT_MANUAL_COMPLETION: &str = "manual_completion";
pub const REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT: &str = "administrative_adjustment";

pub const REWARD_SOURCE_COURSE: &str = "course";
pub const REWARD_SOURCE_ORGANIZATION: &str = "organization";

pub const REWARD_STATUS_PENDING_TEACHER_APPROVAL: &str = "pending_teacher_approval";
pub const REWARD_STATUS_TEACHER_APPROVED: &str = "teacher_approved";
pub const REWARD_STATUS_TEACHER_REJECTED: &str = "teacher_rejected";
pub const REWARD_STATUS_AMOUNT_APPROVED: &str = "amount_approved";
pub const REWARD_STATUS_AMOUNT_REJECTED: &str = "amount_rejected";
pub const REWARD_STATUS_ADJUSTED: &str = "adjusted";
pub const REWARD_STATUS_TOKEN_PENDING: &str = "token_pending";
pub const REWARD_STATUS_TOKEN_CONFIRMED: &str = "token_confirmed";
pub const REWARD_STATUS_WALLET_CREDITED: &str = "wallet_credited";
pub const REWARD_STATUS_NOTIFIED: &str = "notified";
pub const REWARD_STATUS_COMPLETED: &str = "completed";
pub const REWARD_STATUS_NEEDS_RECONCILIATION: &str = "needs_reconciliation";
pub const REWARD_STATUS_FAILED: &str = "failed";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_candidates)]
pub struct RewardCandidate {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub idempotency_key: String,
    pub evidence: Value,
    pub status: String,
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

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_candidates)]
pub struct NewRewardCandidate {
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub idempotency_key: String,
    pub evidence: Value,
    pub status: String,
}
