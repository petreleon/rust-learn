use chrono::{DateTime, Utc};

use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct CourseRewardCandidate {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: String,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub idempotency_key: String,
    pub evidence: JsonValue,
    pub status: String,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: Option<String>,
    pub teacher_decided_at: Option<DateTime<Utc>>,
    pub amount_reviewer_user_id: Option<i32>,
    pub approved_amount: Option<String>,
    pub amount_decision_reason: Option<String>,
    pub amount_decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
