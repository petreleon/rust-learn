use futures::future::{BoxFuture, FutureExt};

use crate::application::content::inspect_processing_history::{
    self, ContentProcessingHistoryError, ContentProcessingHistoryOutput,
    ContentProcessingHistoryQuery, ContentProcessingHistoryUseCase,
};
use crate::infra::postgres::content::processing_history_store::PostgresContentProcessingHistoryStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresContentProcessingHistoryUseCase {
    pool: DbPool,
}

impl PostgresContentProcessingHistoryUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ContentProcessingHistoryUseCase for PostgresContentProcessingHistoryUseCase {
    fn inspect_content_processing_history(
        &self,
        query: ContentProcessingHistoryQuery,
    ) -> BoxFuture<'_, Result<ContentProcessingHistoryOutput, ContentProcessingHistoryError>> {
        async move {
            let mut conn =
                self.pool.get().await.map_err(|error| {
                    ContentProcessingHistoryError::Connection(error.to_string())
                })?;
            let mut store = PostgresContentProcessingHistoryStore::new(&mut conn);
            inspect_processing_history::inspect_content_processing_history(&mut store, query).await
        }
        .boxed()
    }
}
