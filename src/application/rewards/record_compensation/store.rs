use futures::future::BoxFuture;

use crate::application::rewards::record_compensation::{
    RecordRewardCompensationCommand, RewardCompensationError, RewardCompensationOutput,
};

pub struct RewardCompensation {
    pub actor_user_id: i32,
    pub command: RecordRewardCompensationCommand,
}

pub trait RewardCompensationStore {
    fn can_record_reward_compensation(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCompensationError>>;

    fn record_reward_compensation(
        &mut self,
        compensation: RewardCompensation,
    ) -> BoxFuture<'_, Result<RewardCompensationOutput, RewardCompensationError>>;
}
