use crate::application::rewards::reconcile_candidate::{
    RewardReconciliation, RewardReconciliationError, RewardReconciliationOutput,
    RewardReconciliationStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn reconcile_reward_candidate(
    store: &mut impl RewardReconciliationStore,
    candidate_id: i64,
) -> Result<RewardReconciliationOutput, RewardReconciliationError> {
    let result = store
        .reconcile_reward_candidate(RewardReconciliation {
            candidate_id,
            actor_user_id: None,
        })
        .await?;
    log_reconciliation(&result);
    Ok(result)
}

pub async fn reconcile_reward_candidate_for_actor(
    store: &mut impl RewardReconciliationStore,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardReconciliationOutput, RewardReconciliationError> {
    ensure_can_execute_reward_payout(store, actor_user_id).await?;
    let result = store
        .reconcile_reward_candidate(RewardReconciliation {
            candidate_id,
            actor_user_id: Some(actor_user_id),
        })
        .await?;
    log_reconciliation(&result);
    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    store: &mut impl RewardReconciliationStore,
    actor_user_id: i32,
) -> Result<(), RewardReconciliationError> {
    if store.can_execute_reward_payout(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardReconciliationError::PermissionDenied(
            Permissions::EXECUTE_REWARD_PAYOUT.into(),
        ))
    }
}

fn log_reconciliation(result: &RewardReconciliationOutput) {
    log::info!(
        "event=reward_reconciled candidate_id={} wallet_credit_created={} notification_created={} external_transaction_link_repaired={} internal_transaction_link_repaired={} final_status={}",
        result.candidate_id,
        result.wallet_credit_created,
        result.notification_created,
        result.external_transaction_link_repaired,
        result.internal_transaction_link_repaired,
        result.final_status
    );
}

#[cfg(test)]
mod tests;
