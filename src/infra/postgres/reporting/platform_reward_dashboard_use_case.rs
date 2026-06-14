use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_reward_dashboard::{
    self, PlatformRewardDashboardError, PlatformRewardDashboardOutput,
    PlatformRewardDashboardUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::reporting::platform_reward_dashboard_store::PostgresPlatformRewardDashboardStore;

#[derive(Clone)]
pub struct PostgresPlatformRewardDashboardUseCase {
    pool: DbPool,
}

impl PostgresPlatformRewardDashboardUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformRewardDashboardUseCase for PostgresPlatformRewardDashboardUseCase {
    fn load_platform_reward_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformRewardDashboardStore::new(&mut conn);
            platform_reward_dashboard::load_platform_reward_dashboard(&mut store).await
        }
        .boxed()
    }
}

impl PostgresPlatformRewardDashboardUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformRewardDashboardError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformRewardDashboardError::Connection(error.to_string()))
    }
}
