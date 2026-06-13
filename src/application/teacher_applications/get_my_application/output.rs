use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherApplicationSelfOutput {
    pub application: Option<TeacherApplicationOutput>,
    pub audit_events: Vec<TeacherApplicationAuditEventOutput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherApplicationOutput {
    pub id: i64,
    pub applicant_user_id: i32,
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: Value,
    pub status: String,
    pub reviewer_id: Option<i32>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationAuditEventOutput {
    pub id: i64,
    pub application_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
