use futures::future::BoxFuture;

use crate::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput,
};

pub trait OrganizationSummaryStore {
    fn load_organization_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>>;
}
