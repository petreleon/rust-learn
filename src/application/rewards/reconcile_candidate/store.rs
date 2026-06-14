use futures::future::BoxFuture;

use crate::application::rewards::reconcile_candidate::{
    RewardReconciliationError, RewardReconciliationOutput,
};

pub struct RewardReconciliation {
    pub candidate_id: i64,
    pub actor_user_id: Option<i32>,
}

pub trait RewardReconciliationStore {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardReconciliationError>>;

    fn reconcile_reward_candidate(
        &mut self,
        reconciliation: RewardReconciliation,
    ) -> BoxFuture<'_, Result<RewardReconciliationOutput, RewardReconciliationError>>;
}
