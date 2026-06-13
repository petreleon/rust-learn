use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::organizations::list_organization_member_audit::OrganizationMemberAuditEventOutput;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationMemberAuditEventResponse {
    pub id: i64,
    pub organization_id: i32,
    pub actor_user_id: Option<i32>,
    pub target_user_id: i32,
    pub event_type: String,
    pub role_name: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<OrganizationMemberAuditEventOutput> for OrganizationMemberAuditEventResponse {
    fn from(event: OrganizationMemberAuditEventOutput) -> Self {
        Self {
            id: event.id,
            organization_id: event.organization_id,
            actor_user_id: event.actor_user_id,
            target_user_id: event.target_user_id,
            event_type: event.event_type,
            role_name: event.role_name,
            reason: event.reason,
            created_at: event.created_at,
        }
    }
}
