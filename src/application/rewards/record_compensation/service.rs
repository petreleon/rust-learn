use futures::future::BoxFuture;

use crate::application::rewards::record_compensation::{
    RecordRewardCompensationCommand, RewardCompensationError, RewardCompensationOutput,
};

pub trait RewardCompensationUseCase: Send + Sync {
    fn record_reward_compensation(
        &self,
        actor_user_id: i32,
        command: RecordRewardCompensationCommand,
    ) -> BoxFuture<'_, Result<RewardCompensationOutput, RewardCompensationError>>;
}
