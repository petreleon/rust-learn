use crate::infra::postgres::schema::organization_member_audit_events;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug, Clone, serde::Serialize)]
#[diesel(table_name = organization_member_audit_events)]
pub struct OrganizationMemberAuditEvent {
    pub id: i64,
    pub organization_id: i32,
    pub actor_user_id: Option<i32>,
    pub target_user_id: i32,
    pub event_type: String,
    pub role_name: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = organization_member_audit_events)]
pub struct NewOrganizationMemberAuditEvent {
    pub organization_id: i32,
    pub actor_user_id: Option<i32>,
    pub target_user_id: i32,
    pub event_type: String,
    pub role_name: Option<String>,
    pub reason: Option<String>,
}
