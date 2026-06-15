use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_platform_candidates::{
    self, PlatformRewardCandidatesError, PlatformRewardCandidatesOutput,
    PlatformRewardCandidatesQuery, PlatformRewardCandidatesUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::platform_reward_candidate_store::PostgresPlatformRewardCandidateStore;

#[derive(Clone)]
pub struct PostgresPlatformRewardCandidatesUseCase {
    pool: DbPool,
}

impl PostgresPlatformRewardCandidatesUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformRewardCandidatesUseCase for PostgresPlatformRewardCandidatesUseCase {
    fn list_platform_reward_candidates(
        &self,
        actor_user_id: i32,
        query: PlatformRewardCandidatesQuery,
    ) -> BoxFuture<'_, Result<PlatformRewardCandidatesOutput, PlatformRewardCandidatesError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformRewardCandidateStore::new(&mut conn);
            list_platform_candidates::list_platform_reward_candidates(
                &mut store,
                actor_user_id,
                query,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresPlatformRewardCandidatesUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformRewardCandidatesError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformRewardCandidatesError::Connection(error.to_string()))
    }
}
