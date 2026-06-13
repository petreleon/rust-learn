use crate::db::schema::reward_candidates;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

pub const REWARD_EVENT_ASSESSMENT_COMPLETION: &str =
    crate::domain::rewards::candidate::event_type::REWARD_EVENT_ASSESSMENT_COMPLETION;
pub const REWARD_EVENT_COURSE_COMPLETION: &str =
    crate::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub const REWARD_EVENT_MANUAL_COMPLETION: &str =
    crate::domain::rewards::candidate::event_type::REWARD_EVENT_MANUAL_COMPLETION;
pub const REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT: &str =
    crate::domain::rewards::candidate::event_type::REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT;

pub const REWARD_SOURCE_COURSE: &str =
    crate::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub const REWARD_SOURCE_ORGANIZATION: &str =
    crate::domain::rewards::candidate::source::REWARD_SOURCE_ORGANIZATION;

pub const REWARD_STATUS_PENDING_TEACHER_APPROVAL: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_PENDING_TEACHER_APPROVAL;
pub const REWARD_STATUS_TEACHER_APPROVED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_TEACHER_APPROVED;
pub const REWARD_STATUS_TEACHER_REJECTED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_TEACHER_REJECTED;
pub const REWARD_STATUS_AMOUNT_APPROVED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_AMOUNT_APPROVED;
pub const REWARD_STATUS_AMOUNT_REJECTED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_AMOUNT_REJECTED;
#[allow(dead_code)]
pub const REWARD_STATUS_ADJUSTED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_ADJUSTED;
pub const REWARD_STATUS_TOKEN_PENDING: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_TOKEN_PENDING;
pub const REWARD_STATUS_TOKEN_CONFIRMED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_TOKEN_CONFIRMED;
pub const REWARD_STATUS_WALLET_CREDITED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_WALLET_CREDITED;
pub const REWARD_STATUS_NOTIFIED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_NOTIFIED;
pub const REWARD_STATUS_COMPLETED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_COMPLETED;
pub const REWARD_STATUS_NEEDS_RECONCILIATION: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_NEEDS_RECONCILIATION;
pub const REWARD_STATUS_FAILED: &str =
    crate::domain::rewards::candidate::status::REWARD_STATUS_FAILED;

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
