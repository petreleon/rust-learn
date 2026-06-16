use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::manage_reward_policy::{
    self, CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyAuditEventOutput,
    RewardPolicyError, RewardPolicyOutput, RewardPolicyUseCase,
    UpdateRewardPolicyActivationCommand,
};
use crate::infra::postgres::rewards::reward_policy_store::PostgresRewardPolicyStore;
use crate::infra::postgres::DbPool;

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

    fn update_reward_policy_activation(
        &self,
        actor_user_id: i32,
        command: UpdateRewardPolicyActivationCommand,
    ) -> BoxFuture<'_, Result<RewardPolicyOutput, RewardPolicyError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPolicyStore::new(&mut conn);
            manage_reward_policy::update_reward_policy_activation(
                &mut store,
                actor_user_id,
                command,
            )
            .await
        }
        .boxed()
    }

    fn list_reward_policy_audit(
        &self,
        actor_user_id: i32,
        policy_id: i64,
    ) -> BoxFuture<'_, Result<Vec<RewardPolicyAuditEventOutput>, RewardPolicyError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRewardPolicyStore::new(&mut conn);
            manage_reward_policy::list_reward_policy_audit(&mut store, actor_user_id, policy_id)
                .await
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
