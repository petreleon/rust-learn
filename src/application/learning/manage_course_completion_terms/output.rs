use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::domain::learning::course::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsOutput {
    pub id: i64,
    pub course_id: i32,
    pub teacher_user_id: i32,
    pub organization_id: Option<i32>,
    pub version: i32,
    pub status: CourseCompletionTermsStatus,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub reward_policy_id: Option<i64>,
    pub proposed_by_user_id: i32,
    pub accepted_by_user_id: Option<i32>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub activated_at: Option<DateTime<Utc>>,
    pub superseded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsAuditEventOutput {
    pub id: i64,
    pub terms_id: i64,
    pub course_id: i32,
    pub actor_user_id: i32,
    pub event_type: CourseCompletionTermsAuditEventType,
    pub previous_status: Option<CourseCompletionTermsStatus>,
    pub new_status: CourseCompletionTermsStatus,
    pub completion_reward_amount: BigDecimal,
    pub max_enrolled_students: i32,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsHistoryOutput {
    pub active_terms: Option<CourseCompletionTermsOutput>,
    pub terms: Vec<CourseCompletionTermsOutput>,
    pub audit_events: Vec<CourseCompletionTermsAuditEventOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsCourseContext {
    pub course_id: i32,
    pub organization_id: Option<i32>,
}
