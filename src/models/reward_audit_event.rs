use crate::db::schema::reward_audit_events;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = reward_audit_events)]
pub struct RewardAuditEvent {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reward_audit_events)]
pub struct NewRewardAuditEvent {
    pub reward_candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: Value,
}
