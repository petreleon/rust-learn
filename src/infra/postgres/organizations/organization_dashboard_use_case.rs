use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::get_organization_dashboard::{
    self, OrganizationDashboardError, OrganizationDashboardOutput, OrganizationDashboardQuery,
    OrganizationDashboardUseCase,
};
use crate::infra::postgres::organizations::organization_dashboard_store::PostgresOrganizationDashboardStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresOrganizationDashboardUseCase {
    pool: DbPool,
}

impl PostgresOrganizationDashboardUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationDashboardUseCase for PostgresOrganizationDashboardUseCase {
    fn get_organization_dashboard(
        &self,
        query: OrganizationDashboardQuery,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOutput, OrganizationDashboardError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationDashboardStore::new(&mut conn);
            get_organization_dashboard::get_organization_dashboard(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresOrganizationDashboardUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationDashboardError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationDashboardError::Connection(error.to_string()))
    }
}
