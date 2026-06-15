use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_fraud_block::{
    self, CreateRewardFraudBlockCommand, ListRewardFraudBlocksOutput, ListRewardFraudBlocksQuery,
    RewardFraudBlockAuditEventOutput, RewardFraudBlockError, RewardFraudBlockOutput,
    RewardFraudBlockUseCase,
};
use crate::infra::postgres::rewards::reward_fraud_block_store::PostgresRewardFraudBlockStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresRewardFraudBlockUseCase {
    pool: DbPool,
}

impl PostgresRewardFraudBlockUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardFraudBlockUseCase for PostgresRewardFraudBlockUseCase {
    fn create_reward_fraud_block(
        &self,
        actor_user_id: i32,
        command: CreateRewardFraudBlockCommand,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardFraudBlockStore::new(&mut conn);
            manage_fraud_block::create_reward_fraud_block(&mut store, actor_user_id, command).await
        }
        .boxed()
    }

    fn list_reward_fraud_blocks(
        &self,
        actor_user_id: i32,
        query: ListRewardFraudBlocksQuery,
    ) -> BoxFuture<'_, Result<ListRewardFraudBlocksOutput, RewardFraudBlockError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardFraudBlockStore::new(&mut conn);
            manage_fraud_block::list_reward_fraud_blocks(&mut store, actor_user_id, query).await
        }
        .boxed()
    }

    fn revoke_reward_fraud_block(
        &self,
        actor_user_id: i32,
        block_id: i64,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardFraudBlockStore::new(&mut conn);
            manage_fraud_block::revoke_reward_fraud_block(&mut store, actor_user_id, block_id).await
        }
        .boxed()
    }

    fn reward_fraud_block_audit_history(
        &self,
        actor_user_id: i32,
        block_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardFraudBlockAuditEventOutput>, RewardFraudBlockError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardFraudBlockStore::new(&mut conn);
            manage_fraud_block::reward_fraud_block_audit_history(
                &mut store,
                actor_user_id,
                block_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresRewardFraudBlockUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardFraudBlockError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardFraudBlockError::Connection(error.to_string()))
    }
}
