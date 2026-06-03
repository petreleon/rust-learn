use crate::db::schema::reward_audit_events;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

pub const REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED: &str = "candidate_submitted";
pub const REWARD_AUDIT_EVENT_TEACHER_DECISION: &str = "teacher_decision";
pub const REWARD_AUDIT_EVENT_AMOUNT_DECISION: &str = "amount_decision";
pub const REWARD_AUDIT_EVENT_TOKEN_CONFIRMED: &str = "token_confirmed";
pub const REWARD_AUDIT_EVENT_WALLET_CREDITED: &str = "wallet_credited";
pub const REWARD_AUDIT_EVENT_WALLET_CREDIT_NOTIFIED: &str = "wallet_credit_notified";
pub const REWARD_AUDIT_EVENT_RECONCILED: &str = "reconciled";

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
