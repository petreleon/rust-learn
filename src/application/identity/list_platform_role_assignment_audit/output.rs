use chrono::{DateTime, Utc};

use crate::domain::access_control::role_assignment_audit::PlatformRoleAssignmentAuditEventType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRoleAssignmentAuditEventOutput {
    pub id: i64,
    pub target_user_id: i32,
    pub actor_user_id: Option<i32>,
    pub platform_role_id: Option<i32>,
    pub role_name: String,
    pub event_type: PlatformRoleAssignmentAuditEventType,
    pub created_at: DateTime<Utc>,
}
