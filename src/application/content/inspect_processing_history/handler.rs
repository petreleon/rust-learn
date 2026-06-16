use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryError, ContentProcessingHistoryOutput, ContentProcessingHistoryQuery,
};
use crate::application::content::ports::ContentProcessingHistoryStore;

pub async fn inspect_content_processing_history(
    store: &mut impl ContentProcessingHistoryStore,
    query: ContentProcessingHistoryQuery,
) -> Result<ContentProcessingHistoryOutput, ContentProcessingHistoryError> {
    store.processing_history(query).await
}
