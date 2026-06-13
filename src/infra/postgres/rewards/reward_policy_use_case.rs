use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_reward_policy::{
    self, CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyError,
    RewardPolicyOutput, RewardPolicyUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_policy_store::PostgresRewardPolicyStore;

#[derive(Clone)]
pub struct PostgresRewardPolicyUseCase {
    pool: DbPool,
}

impl PostgresRewardPolicyUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RewardPolicyUseCase for PostgresRewardPolicyUseCase {
    fn create_reward_policy(
        &self,
        actor_user_id: i32,
        command: CreateRewardPolicyCommand,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPolicyStore::new(&mut conn);
            manage_reward_policy::create_reward_policy(&mut store, actor_user_id, command).await
        }
        .boxed()
    }

    fn list_reward_policies(
        &self,
        actor_user_id: i32,
        query: ListRewardPoliciesQuery,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyOutput>, RewardPolicyError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPolicyStore::new(&mut conn);
            manage_reward_policy::list_reward_policies(&mut store, actor_user_id, query).await
        }
        .boxed()
    }
}

impl PostgresRewardPolicyUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RewardPolicyError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RewardPolicyError::Connection(error.to_string()))
    }
}
