use futures::future::BoxFuture;

use crate::application::rewards::reconcile_candidate::{
    RewardReconciliationError, RewardReconciliationOutput,
};

pub trait RewardReconciliationUseCase: Send + Sync {
    fn reconcile_reward_candidate(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>>;

    fn reconcile_reward_candidate_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>>;
}
