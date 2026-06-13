use crate::application::rewards::plan_payout::validation::{
    approved_positive_amount, ensure_candidate_ready_for_payout,
};
use crate::application::rewards::plan_payout::{
    RewardPayoutPlan, RewardPayoutPlanError, RewardPayoutPlanStore, RewardPayoutPolicy,
};
use crate::domain::rewards::payout::{
    REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_OFF_CHAIN,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER, REWARD_PAYOUT_METHOD_TREASURY_TRANSFER,
};
use crate::domain::rewards::policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
};

pub async fn plan_reward_payout(
    store: &mut impl RewardPayoutPlanStore,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardPayoutPlanError> {
    let candidate = store.load_candidate(candidate_id).await?;
    ensure_candidate_ready_for_payout(&candidate)?;
    let amount = approved_positive_amount(&candidate)?;
    let policy = store
        .active_policy_for_candidate(candidate.course_id, candidate.event_type)
        .await?
        .ok_or(RewardPayoutPlanError::NoActivePolicy)?;
    let payout_method = select_payout_method(store, &policy).await?;
    let requires_token_confirmation = payout_method != REWARD_PAYOUT_METHOD_OFF_CHAIN;

    let plan = RewardPayoutPlan {
        candidate_id: candidate.id,
        policy_id: policy.id,
        amount,
        payment_strategy: policy.payment_strategy,
        payout_method,
        requires_token_confirmation,
    };

    log::info!(
        "event=reward_payout_planned candidate_id={} policy_id={} amount={} payment_strategy={} payout_method={} requires_token_confirmation={}",
        plan.candidate_id,
        plan.policy_id,
        plan.amount,
        plan.payment_strategy,
        plan.payout_method,
        plan.requires_token_confirmation
    );

    Ok(plan)
}

pub async fn plan_reward_payout_for_actor(
    store: &mut impl RewardPayoutPlanStore,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardPayoutPlan, RewardPayoutPlanError> {
    ensure_can_execute_reward_payout(store, actor_user_id).await?;
    plan_reward_payout(store, candidate_id).await
}

async fn ensure_can_execute_reward_payout(
    store: &mut impl RewardPayoutPlanStore,
    actor_user_id: i32,
) -> Result<(), RewardPayoutPlanError> {
    if store.can_execute_reward_payout(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardPayoutPlanError::PermissionDenied(
            "EXECUTE_REWARD_PAYOUT".to_string(),
        ))
    }
}

async fn select_payout_method(
    store: &mut impl RewardPayoutPlanStore,
    policy: &RewardPayoutPolicy,
) -> Result<String, RewardPayoutPlanError> {
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_TREASURY_TRANSFER => {
            if store.has_presigner_contract().await? {
                Ok(REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER.to_string())
            } else {
                Ok(REWARD_PAYOUT_METHOD_TREASURY_TRANSFER.to_string())
            }
        }
        REWARD_PAYMENT_MINT => Ok(REWARD_PAYOUT_METHOD_MINT.to_string()),
        REWARD_PAYMENT_OFF_CHAIN => Ok(REWARD_PAYOUT_METHOD_OFF_CHAIN.to_string()),
        _ => Err(RewardPayoutPlanError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;
