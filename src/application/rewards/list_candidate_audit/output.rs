use chrono::{DateTime, Utc};

use crate::domain::rewards::audit::{RewardAuditEventType, RewardAuditMetadata};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct RewardCandidateAuditEvent {
    pub id: i64,
    pub reward_candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub event_type: RewardAuditEventType,
    pub from_status: Option<RewardCandidateStatus>,
    pub to_status: RewardCandidateStatus,
    pub reason: Option<String>,
    pub metadata: RewardAuditMetadata,
    pub created_at: DateTime<Utc>,
}
