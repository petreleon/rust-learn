use chrono::{DateTime, Utc};

use crate::domain::rewards::audit::RewardAuditEventType;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::shared::json::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCandidateAuditEvent {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: RewardAuditEventType,
    pub from_status: Option<RewardCandidateStatus>,
    pub to_status: RewardCandidateStatus,
    pub reason: Option<String>,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}
