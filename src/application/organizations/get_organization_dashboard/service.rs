use futures::future::BoxFuture;

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardOutput, OrganizationDashboardQuery,
};

pub trait OrganizationDashboardUseCase: Send + Sync {
    fn get_organization_dashboard(
        &self,
        query: OrganizationDashboardQuery,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOutput, OrganizationDashboardError>>;
}
