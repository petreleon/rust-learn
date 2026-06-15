use futures::future::{BoxFuture, FutureExt};

use crate::application::content::request_media_url::{
    self, ContentMediaUrlError, ContentMediaUrlUseCase, MediaUrlOutput, RequestMediaUrlCommand,
};
use crate::infra::object_storage::content::media_url_provider::S3ContentMediaUrlProvider;
use crate::infra::object_storage::S3State;
use crate::infra::postgres::content::media_object_store::PostgresContentMediaStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresContentMediaUrlUseCase {
    pool: DbPool,
    s3: Option<S3State>,
}

impl PostgresContentMediaUrlUseCase {
    pub fn new(pool: DbPool, s3: S3State) -> Self {
        Self { pool, s3: Some(s3) }
    }

    pub fn from_env(pool: DbPool) -> Self {
        Self { pool, s3: None }
    }
}

impl ContentMediaUrlUseCase for PostgresContentMediaUrlUseCase {
    fn request_media_url(
        &self,
        command: RequestMediaUrlCommand,
    ) -> BoxFuture<'_, Result<MediaUrlOutput, ContentMediaUrlError>> {
        async move {
            let mut conn = self
                .pool
                .get()
                .await
                .map_err(|error| ContentMediaUrlError::Connection(error.to_string()))?;
            let mut store = PostgresContentMediaStore::new(&mut conn);
            let mut media_provider = self.media_provider();

            request_media_url::request_media_url(&mut store, &mut media_provider, command).await
        }
        .boxed()
    }
}

impl PostgresContentMediaUrlUseCase {
    fn media_provider(&self) -> S3ContentMediaUrlProvider {
        match &self.s3 {
            Some(s3) => S3ContentMediaUrlProvider::new(s3.clone()),
            None => S3ContentMediaUrlProvider::from_env(),
        }
    }
}
