use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::infra::postgres::schema::platform_role_assignment_audit_events;

#[derive(Queryable, Identifiable, Debug, Clone, serde::Serialize)]
#[diesel(table_name = platform_role_assignment_audit_events)]
pub struct PlatformRoleAssignmentAuditEvent {
    pub id: i64,
    pub target_user_id: i32,
    pub actor_user_id: Option<i32>,
    pub platform_role_id: Option<i32>,
    pub role_name: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = platform_role_assignment_audit_events)]
pub struct NewPlatformRoleAssignmentAuditEvent {
    pub target_user_id: i32,
    pub actor_user_id: Option<i32>,
    pub platform_role_id: Option<i32>,
    pub role_name: String,
    pub event_type: String,
}
