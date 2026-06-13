use futures::future::BoxFuture;

use crate::application::rewards::record_token_confirmation::{
    RewardTokenConfirmationCommand, RewardTokenConfirmationError, RewardTokenConfirmationOutput,
};

pub trait RewardTokenConfirmationUseCase: Send + Sync {
    fn record_reward_token_confirmation(
        &self,
        candidate_id: i64,
        command: RewardTokenConfirmationCommand,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>>;

    fn record_reward_token_confirmation_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
        command: RewardTokenConfirmationCommand,
    ) -> BoxFuture<'_, Result<RewardTokenConfirmationOutput, RewardTokenConfirmationError>>;
}
