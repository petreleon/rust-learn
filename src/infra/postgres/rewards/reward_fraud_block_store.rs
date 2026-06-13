use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockDraft, RewardFraudBlockError, RewardFraudBlockListFilter,
    RewardFraudBlockOutput,
};
use crate::application::rewards::ports::RewardFraudBlockStore;
use crate::infra::postgres::rewards::reward_fraud_block_mappers::{
    map_reward_fraud_block_error, new_reward_fraud_block,
};
use crate::infra::postgres::rewards::reward_fraud_block_notifications::notify_reward_fraud_block_transition;
use crate::infra::postgres::rewards::reward_fraud_block_permissions::{
    can_manage_fraud_block_scope, can_view_fraud_blocks,
};
use crate::repositories::reward_fraud_block_repository::{self, RewardFraudBlockFilter};

pub struct PostgresRewardFraudBlockStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardFraudBlockStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardFraudBlockStore for PostgresRewardFraudBlockStore<'_> {
    fn can_manage_fraud_block_scope<'a>(
        &'a mut self,
        actor_user_id: i32,
        scope_type: &'a str,
    ) -> BoxFuture<'a, Result<bool, RewardFraudBlockError>> {
        async move { can_manage_fraud_block_scope(self.conn, actor_user_id, scope_type).await }
            .boxed()
    }

    fn can_view_fraud_blocks(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardFraudBlockError>> {
        async move { can_view_fraud_blocks(self.conn, actor_user_id).await }.boxed()
    }

    fn create_fraud_block(
        &mut self,
        draft: RewardFraudBlockDraft,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        async move {
            reward_fraud_block_repository::create_reward_fraud_block(
                self.conn,
                new_reward_fraud_block(draft),
            )
            .await
            .map(RewardFraudBlockOutput::from)
            .map_err(map_reward_fraud_block_error)
        }
        .boxed()
    }

    fn find_fraud_block(
        &mut self,
        block_id: i64,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        async move {
            reward_fraud_block_repository::find_reward_fraud_block(self.conn, block_id)
                .await
                .map(RewardFraudBlockOutput::from)
                .map_err(map_reward_fraud_block_error)
        }
        .boxed()
    }

    fn list_fraud_blocks(
        &mut self,
        filter: RewardFraudBlockListFilter,
    ) -> BoxFuture<'_, Result<(Vec<RewardFraudBlockOutput>, i64), RewardFraudBlockError>> {
        async move {
            reward_fraud_block_repository::list_reward_fraud_blocks(
                self.conn,
                RewardFraudBlockFilter::from(filter),
            )
            .await
            .map(|(blocks, total)| {
                (
                    blocks
                        .into_iter()
                        .map(RewardFraudBlockOutput::from)
                        .collect(),
                    total,
                )
            })
            .map_err(map_reward_fraud_block_error)
        }
        .boxed()
    }

    fn revoke_fraud_block(
        &mut self,
        block_id: i64,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<RewardFraudBlockOutput, RewardFraudBlockError>> {
        async move {
            reward_fraud_block_repository::revoke_reward_fraud_block(
                self.conn,
                block_id,
                actor_user_id,
                chrono::Utc::now(),
            )
            .await
            .map(RewardFraudBlockOutput::from)
            .map_err(map_reward_fraud_block_error)
        }
        .boxed()
    }

    fn notify_fraud_block_transition<'a>(
        &'a mut self,
        block: &'a RewardFraudBlockOutput,
        event_type: &'a str,
    ) -> BoxFuture<'a, Result<(), RewardFraudBlockError>> {
        async move { notify_reward_fraud_block_transition(self.conn, block, event_type).await }
            .boxed()
    }
}
