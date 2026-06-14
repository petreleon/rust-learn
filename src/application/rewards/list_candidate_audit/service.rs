use futures::future::BoxFuture;

use crate::application::rewards::list_candidate_audit::{
    RewardCandidateAuditError, RewardCandidateAuditEvent,
};

pub trait RewardCandidateAuditUseCase: Send + Sync {
    fn list_reward_candidate_audit(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardCandidateAuditEvent>, RewardCandidateAuditError>>;
}
