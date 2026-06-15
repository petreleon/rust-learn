use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};
use crate::application::rewards::ports::RewardCandidateAuditStore;
use crate::domain::access_control::permissions::Permissions;

pub async fn list_reward_candidate_audit(
    store: &mut impl RewardCandidateAuditStore,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError> {
    ensure_can_view_reward_audit(store, actor_user_id).await?;
    store.reward_candidate_exists(candidate_id).await?;
    store.list_candidate_audit_events(candidate_id).await
}

async fn ensure_can_view_reward_audit(
    store: &mut impl RewardCandidateAuditStore,
    actor_user_id: i32,
) -> Result<(), RewardCandidateAuditError> {
    if store.can_view_reward_audit(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardCandidateAuditError::PermissionDenied(
            Permissions::VIEW_REWARD_AUDIT.into(),
        ))
    }
}

#[cfg(test)]
mod tests;
