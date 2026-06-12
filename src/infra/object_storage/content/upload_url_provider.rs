use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentUploadUrlProvider;
use crate::application::content::request_upload_url::ContentUploadUrlError;
use crate::utils::s3_utils::S3State;

pub struct S3ContentUploadUrlProvider {
    state: S3State,
}

impl S3ContentUploadUrlProvider {
    pub fn new(state: S3State) -> Self {
        Self { state }
    }

    pub async fn new_from_env() -> Result<Self, String> {
        S3State::new_from_env()
            .await
            .map(Self::new)
            .map_err(|err| err.to_string())
    }
}

impl ContentUploadUrlProvider for S3ContentUploadUrlProvider {
    fn prepare_upload_url(
        &mut self,
        bucket: &'static str,
        object_key: String,
        expires_seconds: u64,
    ) -> BoxFuture<'_, Result<String, ContentUploadUrlError>> {
        let state = self.state.clone();
        async move {
            state
                .ensure_bucket(bucket)
                .await
                .map_err(|err| ContentUploadUrlError::BucketPrepareFailed(err.to_string()))?;
            state
                .presign_external_put(bucket, &object_key, expires_seconds)
                .await
                .map_err(|err| ContentUploadUrlError::PresignFailed(err.to_string()))
        }
        .boxed()
    }
}
