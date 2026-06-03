use crate::db::schema::{teacher_application_audit_events, teacher_applications};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

pub const TEACHER_APPLICATION_STATUS_SUBMITTED: &str = "submitted";
pub const TEACHER_APPLICATION_STATUS_NEEDS_CHANGES: &str = "needs_changes";
pub const TEACHER_APPLICATION_STATUS_APPROVED: &str = "approved";
pub const TEACHER_APPLICATION_STATUS_REJECTED: &str = "rejected";

pub const TEACHER_APPLICATION_SCOPE_PLATFORM: &str = "platform";
pub const TEACHER_APPLICATION_SCOPE_ORGANIZATION: &str = "organization";
pub const TEACHER_APPLICATION_SCOPE_COURSE: &str = "course";

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = teacher_applications)]
pub struct TeacherApplication {
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

#[derive(Insertable, Debug)]
#[diesel(table_name = teacher_applications)]
pub struct NewTeacherApplication {
    pub applicant_user_id: i32,
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: Value,
    pub status: String,
    pub idempotency_key: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = teacher_application_audit_events)]
pub struct TeacherApplicationAuditEvent {
    pub id: i64,
    pub application_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = teacher_application_audit_events)]
pub struct NewTeacherApplicationAuditEvent {
    pub application_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
}
