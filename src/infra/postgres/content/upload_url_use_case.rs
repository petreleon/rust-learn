use futures::future::{BoxFuture, FutureExt};

use crate::application::content::request_upload_url::{
    self, ContentUploadUrlError, ContentUploadUrlUseCase, RequestUploadUrlCommand, UploadUrlOutput,
};
use crate::db::DbPool;
use crate::infra::object_storage::content::upload_url_provider::S3ContentUploadUrlProvider;
use crate::infra::object_storage::S3State;
use crate::infra::postgres::content::upload_scope_store::PostgresContentUploadScopeStore;

#[derive(Clone)]
pub struct PostgresContentUploadUrlUseCase {
    pool: DbPool,
    s3: Option<S3State>,
}

impl PostgresContentUploadUrlUseCase {
    pub fn new(pool: DbPool, s3: S3State) -> Self {
        Self { pool, s3: Some(s3) }
    }

    pub fn from_env(pool: DbPool) -> Self {
        Self { pool, s3: None }
    }
}

impl ContentUploadUrlUseCase for PostgresContentUploadUrlUseCase {
    fn request_upload_url(
        &self,
        command: RequestUploadUrlCommand,
    ) -> BoxFuture<'_, Result<UploadUrlOutput, ContentUploadUrlError>> {
        async move {
            let mut conn = self
                .pool
                .get()
                .await
                .map_err(|error| ContentUploadUrlError::Connection(error.to_string()))?;
            let mut scope_store = PostgresContentUploadScopeStore::new(&mut conn);
            let mut upload_provider = self.upload_provider().await?;

            request_upload_url::request_upload_url(&mut scope_store, &mut upload_provider, command)
                .await
        }
        .boxed()
    }
}

impl PostgresContentUploadUrlUseCase {
    async fn upload_provider(&self) -> Result<S3ContentUploadUrlProvider, ContentUploadUrlError> {
        match &self.s3 {
            Some(s3) => Ok(S3ContentUploadUrlProvider::new(s3.clone())),
            None => S3ContentUploadUrlProvider::new_from_env()
                .await
                .map_err(ContentUploadUrlError::StorageClientInitFailed),
        }
    }
}
