use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::rewards::list_candidate_audit::RewardCandidateAuditEvent;
use crate::shared::json::JsonValue;

#[derive(Debug, Clone, Serialize)]
pub struct RewardCandidateAuditEventResponse {
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

impl From<RewardCandidateAuditEvent> for RewardCandidateAuditEventResponse {
    fn from(event: RewardCandidateAuditEvent) -> Self {
        Self {
            id: event.id,
            reward_candidate_id: event.reward_candidate_id,
            actor_user_id: event.actor_user_id,
            event_type: event.event_type.as_str().to_string(),
            from_status: event.from_status.map(|status| status.as_str().to_string()),
            to_status: event.to_status.as_str().to_string(),
            reason: event.reason,
            metadata: event.metadata,
            created_at: event.created_at,
        }
    }
}
