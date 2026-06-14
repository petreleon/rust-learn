use futures::future::BoxFuture;

use crate::application::reporting::platform_summary::{
    PlatformSummaryError, PlatformSummaryOutput,
};

pub trait PlatformSummaryUseCase: Send + Sync {
    fn load_platform_summary(
        &self,
    ) -> BoxFuture<'_, Result<PlatformSummaryOutput, PlatformSummaryError>>;
}
