use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_summary::{
    self, PlatformSummaryError, PlatformSummaryOutput, PlatformSummaryUseCase,
};
use crate::infra::postgres::reporting::platform_summary_store::PostgresPlatformSummaryStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresPlatformSummaryUseCase {
    pool: DbPool,
}

impl PostgresPlatformSummaryUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformSummaryUseCase for PostgresPlatformSummaryUseCase {
    fn load_platform_summary(
        &self,
    ) -> BoxFuture<'_, Result<PlatformSummaryOutput, PlatformSummaryError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformSummaryStore::new(&mut conn);
            platform_summary::load_platform_summary(&mut store).await
        }
        .boxed()
    }
}

impl PostgresPlatformSummaryUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformSummaryError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformSummaryError::Connection(error.to_string()))
    }
}
