use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

use super::state::{configured_client, internal_endpoint_from_env, S3State};

impl S3State {
    /// Build a configured async Client from environment variables.
    pub async fn new_from_env() -> Result<Self> {
        let client = configured_client(internal_endpoint_from_env()).await;
        Ok(S3State(Arc::new(client)))
    }

    /// Probe object storage credentials and network connectivity.
    pub async fn health_check(&self) -> Result<()> {
        self.0.list_buckets().send().await?;
        Ok(())
    }

    /// Ensure a bucket exists, creating it if necessary.
    pub async fn ensure_bucket(&self, bucket: &str) -> Result<()> {
        let exists = self.0.head_bucket().bucket(bucket).send().await.is_ok();

        if !exists {
            self.0.create_bucket().bucket(bucket).send().await?;
        }
        Ok(())
    }

    /// Upload a file from disk to the given bucket/object.
    pub async fn put_object_from_path<P: AsRef<Path> + Send + 'static>(
        &self,
        bucket: &str,
        object: &str,
        path: P,
    ) -> Result<()> {
        self.ensure_bucket(bucket).await?;

        let body = aws_sdk_s3::primitives::ByteStream::from_path(path.as_ref()).await?;
        self.0
            .put_object()
            .bucket(bucket)
            .key(object)
            .body(body)
            .send()
            .await?;
        Ok(())
    }
}
