use chrono::NaiveDate;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_reward_dashboard::{
    self, OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
    OrganizationRewardDashboardUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::reporting::organization_reward_dashboard_store::PostgresOrganizationRewardDashboardStore;

#[derive(Clone)]
pub struct PostgresOrganizationRewardDashboardUseCase {
    pool: DbPool,
}

impl PostgresOrganizationRewardDashboardUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationRewardDashboardUseCase for PostgresOrganizationRewardDashboardUseCase {
    fn load_organization_reward_dashboard(
        &self,
        organization_id: i32,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> BoxFuture<'_, Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationRewardDashboardStore::new(&mut conn);
            organization_reward_dashboard::load_organization_reward_dashboard(
                &mut store,
                organization_id,
                from,
                to,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresOrganizationRewardDashboardUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        OrganizationRewardDashboardError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationRewardDashboardError::Connection(error.to_string()))
    }
}
