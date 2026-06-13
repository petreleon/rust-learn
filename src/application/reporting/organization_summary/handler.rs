use crate::application::reporting::organization_summary::store::OrganizationSummaryStore;
use crate::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput,
};

pub async fn load_organization_summary(
    store: &mut impl OrganizationSummaryStore,
    organization_id: i32,
) -> Result<OrganizationSummaryOutput, OrganizationSummaryError> {
    store.load_organization_summary(organization_id).await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_organization_summary;
    use crate::application::reporting::organization_summary::store::OrganizationSummaryStore;
    use crate::application::reporting::organization_summary::{
        OrganizationSummaryError, OrganizationSummaryOutput,
    };

    #[test]
    fn loads_organization_summary_through_store_port() {
        let mut store = FakeOrganizationSummaryStore { requested_id: None };

        let output = block_on(load_organization_summary(&mut store, 42))
            .expect("organization summary should load");

        assert_eq!(store.requested_id, Some(42));
        assert_eq!(output.organization_id, 42);
        assert_eq!(output.course_count, 3);
    }

    struct FakeOrganizationSummaryStore {
        requested_id: Option<i32>,
    }

    impl OrganizationSummaryStore for FakeOrganizationSummaryStore {
        fn load_organization_summary(
            &mut self,
            organization_id: i32,
        ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>> {
            self.requested_id = Some(organization_id);
            ready(Ok(OrganizationSummaryOutput {
                organization_id,
                organization_name: "Org".to_string(),
                course_count: 3,
                member_count: 4,
                wallet_count: 5,
                course_role_assignment_count: 6,
            }))
            .boxed()
        }
    }
}
