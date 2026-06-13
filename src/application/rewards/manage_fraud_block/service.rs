use futures::future::BoxFuture;

use crate::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, ListRewardFraudBlocksOutput, ListRewardFraudBlocksQuery,
    RewardFraudBlockAuditEventOutput, RewardFraudBlockError, RewardFraudBlockOutput,
};

pub trait RewardFraudBlockUseCase: Send + Sync {
    fn create_reward_fraud_block(
        &self,
        actor_user_id: i32,
        command: CreateRewardFraudBlockCommand,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>>;

    fn list_reward_fraud_blocks(
        &self,
        actor_user_id: i32,
        query: ListRewardFraudBlocksQuery,
    ) -> BoxFuture<'_, Result<ListRewardFraudBlocksOutput, RewardFraudBlockError>>;

    fn revoke_reward_fraud_block(
        &self,
        actor_user_id: i32,
        block_id: i64,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>>;

    fn reward_fraud_block_audit_history(
        &self,
        actor_user_id: i32,
        block_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardFraudBlockAuditEventOutput>, RewardFraudBlockError>>;
}
