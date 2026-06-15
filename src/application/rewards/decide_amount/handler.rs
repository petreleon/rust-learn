use crate::application::rewards::decide_amount::validation::{
    approved_amount_for_status, normalize_amount_decision_status,
};
use crate::application::rewards::decide_amount::{
    RewardAmountDecision, RewardAmountDecisionCommand, RewardAmountDecisionError,
    RewardAmountDecisionOutput, RewardAmountDecisionStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn decide_reward_amount(
    store: &mut impl RewardAmountDecisionStore,
    actor_user_id: i32,
    candidate_id: i64,
    command: RewardAmountDecisionCommand,
) -> Result<RewardAmountDecisionOutput, RewardAmountDecisionError> {
    let target_status = normalize_amount_decision_status(&command.status)?;
    ensure_can_approve_reward_amount(store, actor_user_id).await?;
    let approved_amount = approved_amount_for_status(target_status, command.approved_amount)?;

    store
        .decide_reward_amount(RewardAmountDecision {
            actor_user_id,
            candidate_id,
            target_status,
            approved_amount,
            decision_reason: command.decision_reason,
        })
        .await
}

async fn ensure_can_approve_reward_amount(
    store: &mut impl RewardAmountDecisionStore,
    actor_user_id: i32,
) -> Result<(), RewardAmountDecisionError> {
    if store.can_approve_reward_amount(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardAmountDecisionError::PermissionDenied(
            Permissions::APPROVE_REWARD_AMOUNT.into(),
        ))
    }
}

#[cfg(test)]
mod tests;
