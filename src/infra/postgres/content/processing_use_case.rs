use futures::future::{BoxFuture, FutureExt};

use crate::application::content::process_upload_job::{
    self, ContentProcessingUseCase, ProcessUploadJobCommand, ProcessUploadJobError,
    ProcessUploadJobOutput,
};
use crate::db::DbPool;
use crate::infra::postgres::content::upload_job_store::PostgresContentUploadJobStore;

#[derive(Clone)]
pub struct PostgresContentProcessingUseCase {
    pool: DbPool,
}

impl PostgresContentProcessingUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ContentProcessingUseCase for PostgresContentProcessingUseCase {
    fn process_upload_job(
        &self,
        command: ProcessUploadJobCommand,
    ) -> BoxFuture<'_, Result<ProcessUploadJobOutput, ProcessUploadJobError>> {
        async move {
            let mut conn = self
                .pool
                .get()
                .await
                .map_err(|error| ProcessUploadJobError::Connection(error.to_string()))?;
            let mut store = PostgresContentUploadJobStore::new(&mut conn);
            process_upload_job::process_upload_job(&mut store, command).await
        }
        .boxed()
    }
}
