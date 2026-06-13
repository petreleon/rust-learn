use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_fraud_dashboard::{
    self, PlatformFraudDashboardError, PlatformFraudDashboardOutput, PlatformFraudDashboardUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::reporting::platform_fraud_dashboard_store::PostgresPlatformFraudDashboardStore;

#[derive(Clone)]
pub struct PostgresPlatformFraudDashboardUseCase {
    pool: DbPool,
}

impl PostgresPlatformFraudDashboardUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformFraudDashboardUseCase for PostgresPlatformFraudDashboardUseCase {
    fn load_platform_fraud_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformFraudDashboardStore::new(&mut conn);
            platform_fraud_dashboard::load_platform_fraud_dashboard(&mut store).await
        }
        .boxed()
    }
}

impl PostgresPlatformFraudDashboardUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformFraudDashboardError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformFraudDashboardError::Connection(error.to_string()))
    }
}
