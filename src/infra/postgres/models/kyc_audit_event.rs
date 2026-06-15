use crate::infra::postgres::schema::kyc_audit_events;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = kyc_audit_events)]
pub struct KycAuditEvent {
    pub id: i64,
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = kyc_audit_events)]
pub struct NewKycAuditEvent {
    pub submission_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
}
