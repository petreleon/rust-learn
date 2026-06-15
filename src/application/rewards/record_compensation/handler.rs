use crate::application::rewards::record_compensation::validation::validate_compensation_command;
use crate::application::rewards::record_compensation::{
    RecordRewardCompensationCommand, RewardCompensation, RewardCompensationError,
    RewardCompensationOutput, RewardCompensationStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn record_reward_compensation(
    store: &mut impl RewardCompensationStore,
    actor_user_id: i32,
    command: RecordRewardCompensationCommand,
) -> Result<RewardCompensationOutput, RewardCompensationError> {
    ensure_can_record_reward_compensation(store, actor_user_id).await?;
    validate_compensation_command(&command)?;
    store
        .record_reward_compensation(RewardCompensation {
            actor_user_id,
            command,
        })
        .await
}

async fn ensure_can_record_reward_compensation(
    store: &mut impl RewardCompensationStore,
    actor_user_id: i32,
) -> Result<(), RewardCompensationError> {
    if store.can_record_reward_compensation(actor_user_id).await? {
        Ok(())
    } else {
        Err(RewardCompensationError::PermissionDenied(
            Permissions::RECONCILE_WALLETS.into(),
        ))
    }
}

#[cfg(test)]
mod tests;
