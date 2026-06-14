use futures::future::BoxFuture;

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationCommand, RewardTokenConfirmationError, RewardTokenConfirmationOutput,
};

pub struct RewardTokenConfirmation {
    pub candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub command: RewardTokenConfirmationCommand,
    pub transaction_type: String,
}

pub trait RewardTokenConfirmationStore {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardTokenConfirmationError>>;

    fn record_reward_token_confirmation(
        &mut self,
        confirmation: RewardTokenConfirmation,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>>;
}
