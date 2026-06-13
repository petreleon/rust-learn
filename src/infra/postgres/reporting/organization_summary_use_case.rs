use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_summary::{
    self, OrganizationSummaryError, OrganizationSummaryOutput, OrganizationSummaryUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::reporting::organization_summary_store::PostgresOrganizationSummaryStore;

#[derive(Clone)]
pub struct PostgresOrganizationSummaryUseCase {
    pool: DbPool,
}

impl PostgresOrganizationSummaryUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationSummaryUseCase for PostgresOrganizationSummaryUseCase {
    fn load_organization_summary(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationSummaryStore::new(&mut conn);
            organization_summary::load_organization_summary(&mut store, organization_id).await
        }
        .boxed()
    }
}

impl PostgresOrganizationSummaryUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        OrganizationSummaryError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationSummaryError::Connection(error.to_string()))
    }
}
