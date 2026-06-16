use futures::future::BoxFuture;

use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryError, ContentProcessingHistoryOutput, ContentProcessingHistoryQuery,
};

pub trait ContentProcessingHistoryUseCase: Send + Sync {
    fn inspect_content_processing_history(
        &self,
        query: ContentProcessingHistoryQuery,
    ) -> BoxFuture<'_, Result<ContentProcessingHistoryOutput, ContentProcessingHistoryError>>;
}
