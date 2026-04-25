use anyhow::Result;
use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::utils::notifications::NotificationsState;
use chrono::Utc;
use reqwest::Client as ReqwestClient;
use std::process::Stdio;
use tokio::fs as tokio_fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as TokioCommand;

/// Async S3 client wrapper. Works with any S3-compatible service (AWS S3, rus3fs, etc.).
#[derive(Clone)]
pub struct S3State(Arc<Client>);

impl S3State {
    /// Build a configured async Client from environment variables.
    pub async fn new_from_env() -> Result<Self> {
        let user = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let pass = env::var("S3_SECRET_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let host = env::var("S3_INTERNAL_DOMAIN").unwrap_or_else(|_| "rustfs".into());
        let port = env::var("S3_INTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("S3_INTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let creds = Credentials::new(user, pass, None, None, "env");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(creds)
            .endpoint_url(endpoint)
            .load()
            .await;

        let client = Client::new(&config);
        Ok(S3State(Arc::new(client)))
    }

    /// Ensure a bucket exists, creating it if necessary.
    async fn ensure_bucket(&self, bucket: &str) -> Result<()> {
        let exists = self
            .0
            .head_bucket()
            .bucket(bucket)
            .send()
            .await
            .is_ok();

        if !exists {
            self.0
                .create_bucket()
                .bucket(bucket)
                .send()
                .await?;
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
        let user = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let pass = env::var("S3_SECRET_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let host = env::var("S3_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
        let port = env::var("S3_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("S3_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let creds = Credentials::new(user, pass, None, None, "env");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(creds)
            .endpoint_url(endpoint)
            .load()
            .await;

        let client = Client::new(&config);
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
        let url = self.presign_external_put(bucket, object, expires_seconds).await?;
        let mut map = std::collections::HashMap::new();
        map.insert("url".to_string(), url);
        map.insert("key".to_string(), object.to_string());
        Ok(map)
    }

    /// Generate a presigned PUT URL using the external endpoint.
    pub async fn presign_external_put(
        &self,
        bucket: &str,
        object: &str,
        expires_seconds: u64,
    ) -> Result<String> {
        let user = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let pass = env::var("S3_SECRET_KEY").unwrap_or_else(|_| "rustfsadmin".into());
        let host = env::var("S3_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
        let port = env::var("S3_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("S3_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let creds = Credentials::new(user, pass, None, None, "env");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(creds)
            .endpoint_url(endpoint)
            .load()
            .await;

        let client = Client::new(&config);
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

    /// Download an object via a presigned GET URL into a local path.
    async fn download_via_presigned(&self, bucket: &str, object: &str, dst: PathBuf) -> Result<()> {
        let url = self.presign_get(bucket, object, 60).await?; // short lived
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

    /// Process a freshly uploaded video: download it, run ffmpeg to transcode
    /// and extract audio, upload the resulting files back to the same bucket
    /// under a `processed/` prefix, and notify the user via `notifications`.
    pub async fn process_uploaded_video(
        &self,
        bucket: &str,
        object: &str,
        user_id: i32,
        notifications: NotificationsState,
    ) -> Result<()> {
        let tmp_dir = std::env::temp_dir().join(format!(
            "video_process_{}_{}",
            user_id,
            Utc::now().timestamp()
        ));
        tokio_fs::create_dir_all(&tmp_dir).await?;

        let src_path = tmp_dir.join("uploaded_input");
        self.download_via_presigned(bucket, object, src_path.clone())
            .await?;

        let processed_video = tmp_dir.join("processed.mp4");
        let extracted_audio = tmp_dir.join("audio.mp3");

        // transcode to mp4
        let status = TokioCommand::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(src_path.to_string_lossy().as_ref())
            .arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("fast")
            .arg("-c:a")
            .arg("aac")
            .arg(processed_video.to_string_lossy().as_ref())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        if !status.success() {
            let _ = notifications
                .send_notification(
                    user_id,
                    "video:processing_failed",
                    format!("Failed to transcode {}", object),
                )
                .await;
            return Err(anyhow::anyhow!("ffmpeg transcode failed"));
        }

        // extract audio
        let status2 = TokioCommand::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(processed_video.to_string_lossy().as_ref())
            .arg("-q:a")
            .arg("0")
            .arg("-map")
            .arg("a")
            .arg(extracted_audio.to_string_lossy().as_ref())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        if !status2.success() {
            let _ = notifications
                .send_notification(
                    user_id,
                    "video:audio_extraction_failed",
                    format!("Failed to extract audio for {}", object),
                )
                .await;
            return Err(anyhow::anyhow!("ffmpeg audio extraction failed"));
        }

        let processed_object = format!("processed/{}", object);
        let audio_object = format!("processed/audio/{}.mp3", object.replace('/', "_"));

        self.put_object_from_path(bucket, &processed_object, processed_video.clone())
            .await?;
        self.put_object_from_path(bucket, &audio_object, extracted_audio.clone())
            .await?;

        notifications
            .send_notification(
                user_id,
                "video:processed",
                format!(
                    "Your video '{}' has been processed. Video: {} Audio: {}",
                    object, processed_object, audio_object
                ),
            )
            .await?;

        let _ = tokio_fs::remove_dir_all(&tmp_dir).await;
        Ok(())
    }
}
