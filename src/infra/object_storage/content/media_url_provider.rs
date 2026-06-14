use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentMediaUrlProvider;
use crate::application::content::request_media_url::ContentMediaUrlError;
use crate::infra::object_storage::S3State;

pub struct S3ContentMediaUrlProvider {
    state: Option<S3State>,
}

impl S3ContentMediaUrlProvider {
    pub fn new(state: S3State) -> Self {
        Self { state: Some(state) }
    }

    pub fn from_env() -> Self {
        Self { state: None }
    }
}

impl ContentMediaUrlProvider for S3ContentMediaUrlProvider {
    fn media_url(
        &mut self,
        bucket: &'static str,
        object_key: String,
        expires_seconds: u64,
    ) -> BoxFuture<'_, Result<String, ContentMediaUrlError>> {
        let state = self.state.clone();
        async move {
            let state = match state {
                Some(state) => state,
                None => S3State::new_from_env().await.map_err(|err| {
                    ContentMediaUrlError::StorageClientInitFailed(err.to_string())
                })?,
            };

            state
                .presign_external_get(bucket, &object_key, expires_seconds)
                .await
                .map_err(|err| ContentMediaUrlError::PresignFailed {
                    object_key,
                    message: err.to_string(),
                })
        }
        .boxed()
    }
}
