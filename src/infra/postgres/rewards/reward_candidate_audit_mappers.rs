use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::models::reward_audit_event::RewardAuditEvent;

impl From<RewardAuditEvent> for RewardCandidateAuditEvent {
    fn from(event: RewardAuditEvent) -> Self {
        Self {
            id: event.id,
            reward_candidate_id: event.reward_candidate_id,
            actor_user_id: event.actor_user_id,
            event_type: event.event_type,
            from_status: event.from_status,
            to_status: event.to_status,
            reason: event.reason,
            metadata: event.metadata,
            created_at: event.created_at,
        }
    }
}

pub(super) fn map_reward_candidate_audit_error(
    error: diesel::result::Error,
) -> RewardCandidateAuditError {
    match error {
        diesel::result::Error::NotFound => RewardCandidateAuditError::NotFound,
        other => RewardCandidateAuditError::Database(other.to_string()),
    }
}
