use futures::future::BoxFuture;

use crate::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput,
};

pub trait OrganizationSummaryUseCase: Send + Sync {
    fn load_organization_summary(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>>;
}
