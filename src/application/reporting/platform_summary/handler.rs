use crate::application::reporting::platform_summary::store::PlatformSummaryStore;
use crate::application::reporting::platform_summary::{
    PlatformSummaryError, PlatformSummaryOutput,
};

pub async fn load_platform_summary(
    store: &mut impl PlatformSummaryStore,
) -> Result<PlatformSummaryOutput, PlatformSummaryError> {
    store.load_platform_summary().await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_platform_summary;
    use crate::application::reporting::platform_summary::store::PlatformSummaryStore;
    use crate::application::reporting::platform_summary::{
        PlatformSummaryError, PlatformSummaryOutput,
    };

    #[test]
    fn loads_platform_summary_through_store_port() {
        let mut store = FakePlatformSummaryStore { called: false };

        let output =
            block_on(load_platform_summary(&mut store)).expect("platform summary should load");

        assert!(store.called);
        assert_eq!(output.total_users, 10);
        assert_eq!(output.total_courses, 30);
    }

    struct FakePlatformSummaryStore {
        called: bool,
    }

    impl PlatformSummaryStore for FakePlatformSummaryStore {
        fn load_platform_summary(
            &mut self,
        ) -> BoxFuture<'_, Result<PlatformSummaryOutput, PlatformSummaryError>> {
            self.called = true;
            ready(Ok(PlatformSummaryOutput {
                total_users: 10,
                total_organizations: 20,
                total_courses: 30,
                total_wallets: 40,
                total_notifications: 50,
            }))
            .boxed()
        }
    }
}
