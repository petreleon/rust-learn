use crate::application::rewards::credit_wallet::{
    RewardWalletCredit, RewardWalletCreditError, RewardWalletCreditOutput, RewardWalletCreditStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn credit_reward_wallet(
    store: &mut impl RewardWalletCreditStore,
    candidate_id: i64,
) -> Result<RewardWalletCreditOutput, RewardWalletCreditError> {
    let result = store
        .credit_reward_wallet(RewardWalletCredit {
            candidate_id,
            actor_user_id: None,
            allow_reconciliation_credit: false,
        })
        .await?;
    log_wallet_credit(&result, None);
    Ok(result)
}

pub async fn credit_reward_wallet_for_actor(
    store: &mut impl RewardWalletCreditStore,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditOutput, RewardWalletCreditError> {
    ensure_can_execute_reward_payout(store, actor_user_id).await?;
    let result = store
        .credit_reward_wallet(RewardWalletCredit {
            candidate_id,
            actor_user_id: Some(actor_user_id),
            allow_reconciliation_credit: false,
        })
        .await?;
    log_wallet_credit(&result, Some(actor_user_id));
    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    store: &mut impl RewardWalletCreditStore,
    actor_user_id: i32,
) -> Result<(), RewardWalletCreditError> {
    if store.can_execute_reward_payout(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardWalletCreditError::PermissionDenied(
            Permissions::EXECUTE_REWARD_PAYOUT.into(),
        ))
    }
}

fn log_wallet_credit(result: &RewardWalletCreditOutput, actor_user_id: Option<i32>) {
    if let Some(actor_user_id) = actor_user_id {
        log::info!(
            "event=reward_wallet_credit actor_user_id={} candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
            actor_user_id,
            result.candidate_id,
            result.wallet_id,
            result.amount,
            result.credited,
            result.credit_record_id,
            result.transaction_id,
            result.internal_transaction_id
        );
    } else {
        log::info!(
            "event=reward_wallet_credit candidate_id={} wallet_id={} amount={} credited={} credit_record_id={:?} transaction_id={:?} internal_transaction_id={:?}",
            result.candidate_id,
            result.wallet_id,
            result.amount,
            result.credited,
            result.credit_record_id,
            result.transaction_id,
            result.internal_transaction_id
        );
    }
}

#[cfg(test)]
mod tests;
