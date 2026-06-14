use futures::future::BoxFuture;

use crate::application::reporting::platform_summary::{
    PlatformSummaryError, PlatformSummaryOutput,
};

pub trait PlatformSummaryStore {
    fn load_platform_summary(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformSummaryOutput, PlatformSummaryError>>;
}
