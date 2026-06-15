use chrono::{DateTime, Utc};

use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationAuditEventOutput {
    pub id: i64,
    pub application_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: TeacherApplicationAuditEventType,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
