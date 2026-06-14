use anyhow::Result;
use aws_sdk_s3::presigning::PresigningConfig;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use reqwest::Client as ReqwestClient;
use tokio::fs as tokio_fs;
use tokio::io::AsyncWriteExt;

use super::state::{configured_client, S3State};

impl S3State {
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
    ) -> Result<HashMap<String, String>> {
        // AWS SDK for Rust doesn't have a direct PostPolicy equivalent.
        // Use presigned PUT as a workaround for now.
        let url = self.presign_put(bucket, object, expires_seconds).await?;
        let mut map = HashMap::new();
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
        let client = configured_client(external_endpoint_from_env()).await;
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

    /// Generate a presigned PUT URL using the external endpoint.
    pub async fn presign_external_put(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<String> {
        let client = configured_client(external_endpoint_from_env()).await;
        let presign_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_seconds))
            .build()?;

        let url = client
            .put_object()
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
    ) -> Result<HashMap<String, String>> {
        let url = self
            .presign_external_put(bucket, object, expires_seconds)
            .await?;
        let mut map = HashMap::new();
        map.insert("url".to_string(), url);
        map.insert("key".to_string(), object.to_string());
        Ok(map)
    }

    /// Download an object via a presigned GET URL into a local path.
    pub(super) async fn download_via_presigned(
        &self,
        bucket: &str,
        object: &str,
        dst: PathBuf,
    ) -> Result<()> {
        let url = self.presign_get(bucket, object, 60).await?;
        let client = ReqwestClient::new();
        let resp = client.get(&url).send().await?;
        let bytes = resp.bytes().await?;
        if let Some(parent) = dst.parent() {
            tokio_fs::create_dir_all(parent).await?;
        }
        let mut f = tokio_fs::File::create(&dst).await?;
        f.write_all(&bytes).await?;
        Ok(())
    }
}

fn external_endpoint_from_env() -> String {
    let host = env::var("S3_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
    let port = env::var("S3_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
    let scheme = env::var("S3_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
    format!("{}://{}:{}", scheme, host, port)
}
