use chrono::{DateTime, Utc};

use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCandidateAuditEvent {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: String,
    pub from_status: Option<String>,
    pub to_status: String,
    pub reason: Option<String>,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}
