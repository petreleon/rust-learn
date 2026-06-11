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

    /// Generate a presigned GET URL for the given bucket/object.
    pub async fn presign_get(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<String> {
        let config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_seconds))
            .build()?;

        let url = self
            .0
            .get_object()
            .bucket(bucket)
            .key(object)
            .presigned(config)
            .await?;
        Ok(url.uri().to_string())
    }

    /// Generate a presigned PUT URL for uploading an object.
    pub async fn presign_put(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<String> {
        let config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_seconds))
            .build()?;

        let url = self
            .0
            .put_object()
            .bucket(bucket)
            .key(object)
            .presigned(config)
            .await?;
        Ok(url.uri().to_string())
    }

    /// Generate a presigned POST policy for direct browser uploads.
    pub async fn presign_post_form_data(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<std::collections::HashMap<String, String>> {
        // AWS SDK for Rust doesn't have a direct PostPolicy equivalent.
        // Use presigned PUT as a workaround for now.
        let url = self.presign_put(bucket, object, expires_seconds).await?;
        let mut map = std::collections::HashMap::new();
        map.insert("url".to_string(), url);
        map.insert("key".to_string(), object.to_string());
        Ok(map)
    }

    /// Generate a presigned GET URL using the external endpoint.
    pub async fn presign_external_get(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<String> {
        let host = env::var("S3_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
        let port = env::var("S3_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("S3_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let client = configured_client(endpoint).await;
        let presign_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_seconds))
            .build()?;

        let url = client
            .get_object()
            .bucket(bucket)
            .key(object)
            .presigned(presign_config)
            .await?;
        Ok(url.uri().to_string())
    }

    /// Generate presigned POST form data using the external endpoint.
    pub async fn presign_external_post_form_data(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<std::collections::HashMap<String, String>> {
        let url = self
            .presign_external_put(bucket, object, expires_seconds)
            .await?;
        let mut map = std::collections::HashMap::new();
        map.insert("url".to_string(), url);
        map.insert("key".to_string(), object.to_string());
        Ok(map)
    }
}
