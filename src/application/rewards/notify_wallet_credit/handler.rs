use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotification, RewardWalletCreditNotificationError,
    RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn notify_reward_wallet_credit(
    store: &mut impl RewardWalletCreditNotificationStore,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError> {
    let result = store
        .notify_reward_wallet_credit(RewardWalletCreditNotification {
            candidate_id,
            actor_user_id: None,
            allow_reconciliation_repair: false,
        })
        .await?;
    log_wallet_credit_notification(&result, None);
    Ok(result)
}

pub async fn notify_reward_wallet_credit_for_actor(
    store: &mut impl RewardWalletCreditNotificationStore,
    actor_user_id: i32,
    candidate_id: i64,
) -> Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError> {
    ensure_can_execute_reward_payout(store, actor_user_id).await?;
    let result = store
        .notify_reward_wallet_credit(RewardWalletCreditNotification {
            candidate_id,
            actor_user_id: Some(actor_user_id),
            allow_reconciliation_repair: false,
        })
        .await?;
    log_wallet_credit_notification(&result, Some(actor_user_id));
    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    store: &mut impl RewardWalletCreditNotificationStore,
    actor_user_id: i32,
) -> Result<(), RewardWalletCreditNotificationError> {
    if store.can_execute_reward_payout(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardWalletCreditNotificationError::PermissionDenied(
            Permissions::EXECUTE_REWARD_PAYOUT.into(),
        ))
    }
}

fn log_wallet_credit_notification(
    result: &RewardWalletCreditNotificationOutput,
    actor_user_id: Option<i32>,
) {
    if let Some(actor_user_id) = actor_user_id {
        log::info!(
            "event=reward_wallet_credit_notification actor_user_id={} candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
            actor_user_id,
            result.candidate_id,
            result.wallet_id,
            result.amount,
            result.notified,
            result.notification_id,
            result.transaction_id
        );
    } else {
        log::info!(
            "event=reward_wallet_credit_notification candidate_id={} wallet_id={} amount={} notified={} notification_id={:?} transaction_id={}",
            result.candidate_id,
            result.wallet_id,
            result.amount,
            result.notified,
            result.notification_id,
            result.transaction_id
        );
    }
}

#[cfg(test)]
mod tests;
