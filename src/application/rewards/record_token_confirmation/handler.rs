use crate::application::rewards::record_token_confirmation::validation::validate_token_confirmation_command;
use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmation, RewardTokenConfirmationCommand, RewardTokenConfirmationError,
    RewardTokenConfirmationOutput, RewardTokenConfirmationStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn record_reward_token_confirmation(
    store: &mut impl RewardTokenConfirmationStore,
    candidate_id: i64,
    command: RewardTokenConfirmationCommand,
) -> Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError> {
    let transaction_type = validate_token_confirmation_command(&command)?;
    let result = store
        .record_reward_token_confirmation(RewardTokenConfirmation {
            candidate_id,
            actor_user_id: None,
            command,
            transaction_type,
        })
        .await?;
    log_recorded_token_confirmation(&result);
    Ok(result)
}

pub async fn record_reward_token_confirmation_for_actor(
    store: &mut impl RewardTokenConfirmationStore,
    actor_user_id: i32,
    candidate_id: i64,
    command: RewardTokenConfirmationCommand,
) -> Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError> {
    ensure_can_execute_reward_payout(store, actor_user_id).await?;
    let transaction_type = validate_token_confirmation_command(&command)?;
    let result = store
        .record_reward_token_confirmation(RewardTokenConfirmation {
            candidate_id,
            actor_user_id: Some(actor_user_id),
            command,
            transaction_type,
        })
        .await?;
    log_recorded_token_confirmation(&result);
    Ok(result)
}

async fn ensure_can_execute_reward_payout(
    store: &mut impl RewardTokenConfirmationStore,
    actor_user_id: i32,
) -> Result<(), RewardTokenConfirmationError> {
    if store.can_execute_reward_payout(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardTokenConfirmationError::PermissionDenied(
            Permissions::EXECUTE_REWARD_PAYOUT.into(),
        ))
    }
}

fn log_recorded_token_confirmation(result: &RewardTokenConfirmationOutput) {
    log::info!(
        "event=reward_token_confirmed candidate_id={} transaction_id={} external_transaction_id={} payout_record_id={} inserted_external_transaction={}",
        result.candidate_id,
        result.transaction_id,
        result.external_transaction_id,
        result.payout_record_id,
        result.inserted_external_transaction
    );
}

#[cfg(test)]
mod tests;
