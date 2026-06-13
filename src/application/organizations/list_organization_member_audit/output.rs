use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberAuditEventOutput {
    pub id: i64,
    pub organization_id: i32,
    pub actor_user_id: Option<i32>,
    pub target_user_id: i32,
    pub event_type: String,
    pub role_name: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
