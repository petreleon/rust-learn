use std::sync::Arc;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use std::path::{Path, PathBuf};

use minio::s3::{Client, client::ClientBuilder, creds::StaticProvider, builders::{ObjectContent, PostPolicy}};
use minio::s3::types::S3Api;
use minio::s3::builders::GetPresignedPolicyFormData;
use std::collections::HashMap;
use http::Method;

use crate::utils::notifications::NotificationsState;
use tokio::fs as tokio_fs;
use tokio::io::AsyncWriteExt;
use reqwest::Client as ReqwestClient;
use std::process::Stdio;
use tokio::process::Command as TokioCommand;
use chrono::Utc;

/// Async MinIO client wrapper (thin). Uses the async ClientBuilder API from minio v0.3 examples.
#[derive(Clone)]
pub struct MinioState(Arc<Client>);

impl MinioState {
    /// Build a configured async Client from environment variables.
    pub async fn new_from_env() -> Result<Self> {
        dotenv().ok();
        let user = env::var("MINIO_ROOT_USER").unwrap_or_else(|_| "minioadmin".into());
        let pass = env::var("MINIO_ROOT_PASSWORD").unwrap_or_else(|_| "minioadmin".into());
        let host = env::var("MINIO_INTERNAL_DOMAIN").unwrap_or_else(|_| "minio".into());
        let port = env::var("MINIO_INTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("MINIO_INTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let provider = StaticProvider::new(&user, &pass, None);
        let client: Client = ClientBuilder::new(endpoint.parse()?)
            .provider(Some(Box::new(provider)))
            .build()?;

        Ok(MinioState(Arc::new(client)))
    }

    /// Upload a file from disk to the given bucket/object using ObjectContent (example pattern).
    pub async fn put_object_from_path<P: AsRef<Path> + Send + 'static>(&self, bucket: &str, object: &str, path: P) -> Result<()> {
        // Ensure bucket exists and create if not.
        let resp = self.0.bucket_exists(bucket).send().await?;
        if !resp.exists {
            // Try to create the bucket; if it's already owned by us (or was
            // created concurrently) treat that as success.
            match self.0.create_bucket(bucket).send().await {
                Ok(_) => {}
                Err(e) => {
                    let s = e.to_string();
                    if s.contains("BucketAlreadyOwnedByYou") {
                        // ignore
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }

        let content = ObjectContent::from(path.as_ref());
        self.0.put_object_content(bucket, object, content).send().await?;
        Ok(())
    }

    /// Presign GET - if minio crate exposes presign functionality upstream you can implement it here.
    /// Returns a presigned URL for the given bucket/object using HTTP GET.
    /// `expires_seconds` is mapped to the builder if supported by the upstream API.
    pub async fn presign_get(&self, bucket: &str, object: &str, expires_seconds: u64) -> Result<String> {
        let mut builder = self.0.get_presigned_object_url(bucket, object, Method::GET);
        if expires_seconds > 0 {
            let s: u32 = std::cmp::min(expires_seconds, u64::from(u32::MAX)) as u32;
            builder = builder.expiry_seconds(s);
        }

        let resp = builder.send().await?;
        Ok(resp.url)
    }

    /// Create presigned POST form data from a PostPolicy. Returns a map of form
    /// fields that must be included in an HTML form or multipart upload.
    pub async fn presign_post_form_data(&self, policy: PostPolicy) -> Result<HashMap<String, String>> {
        let builder: GetPresignedPolicyFormData = self.0.get_presigned_post_form_data(policy);
        let resp: HashMap<String, String> = builder.send().await?;
        Ok(resp)
    }

    /// Generate a presigned GET URL using the external endpoint configured
    /// in env: MINIO_EXTERNAL_{SCHEME,DOMAIN,PORT} so the returned URL is
    /// signed for the host clients (browser) will use.
    pub async fn presign_external_get(&self, bucket: &str, object: &str, expires_seconds: u64) -> Result<String> {
        dotenv().ok();
        let user = env::var("MINIO_ROOT_USER").unwrap_or_else(|_| "minioadmin".into());
        let pass = env::var("MINIO_ROOT_PASSWORD").unwrap_or_else(|_| "minioadmin".into());
        let host = env::var("MINIO_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
        let port = env::var("MINIO_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("MINIO_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let provider = StaticProvider::new(&user, &pass, None);
        let signer_client: Client = ClientBuilder::new(endpoint.parse()?)
            .provider(Some(Box::new(provider)))
            .build()?;

        let mut builder = signer_client.get_presigned_object_url(bucket, object, Method::GET);
        if expires_seconds > 0 {
            let s: u32 = std::cmp::min(expires_seconds, u64::from(u32::MAX)) as u32;
            builder = builder.expiry_seconds(s);
        }

        let resp = builder.send().await?;
        Ok(resp.url)
    }

    /// Generate presigned POST form data using the external endpoint configured
    /// in env: MINIO_EXTERNAL_{SCHEME,DOMAIN,PORT}. Caller must construct a
    /// `PostPolicy` (with expiration and conditions) and pass it here.
    pub async fn presign_external_post_form_data(&self, policy: PostPolicy) -> Result<HashMap<String, String>> {
        dotenv().ok();
        let user = env::var("MINIO_ROOT_USER").unwrap_or_else(|_| "minioadmin".into());
        let pass = env::var("MINIO_ROOT_PASSWORD").unwrap_or_else(|_| "minioadmin".into());
        let host = env::var("MINIO_EXTERNAL_DOMAIN").unwrap_or_else(|_| "localhost".into());
        let port = env::var("MINIO_EXTERNAL_PORT").unwrap_or_else(|_| "9000".into());
        let scheme = env::var("MINIO_EXTERNAL_SCHEME").unwrap_or_else(|_| "http".into());
        let endpoint = format!("{}://{}:{}", scheme, host, port);

        let provider = StaticProvider::new(&user, &pass, None);
        let signer_client: Client = ClientBuilder::new(endpoint.parse()?)
            .provider(Some(Box::new(provider)))
            .build()?;

        let builder: GetPresignedPolicyFormData = signer_client.get_presigned_post_form_data(policy);
        let resp: HashMap<String, String> = builder.send().await?;
        Ok(resp)
    }

    /// Generate a presigned PUT URL for uploading an object. This returns a URL
    /// the client can PUT to with the file bytes (or use a browser XHR/Fetch).
    pub async fn presign_put(&self, bucket: &str, object: &str, expires_seconds: u64) -> Result<String> {
        let mut builder = self.0.get_presigned_object_url(bucket, object, Method::PUT);
        if expires_seconds > 0 {
            let s: u32 = std::cmp::min(expires_seconds, u64::from(u32::MAX)) as u32;
            builder = builder.expiry_seconds(s);
        }

        let resp = builder.send().await?;
        Ok(resp.url)
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
    ///
    /// Notes:
    /// - This function assumes `ffmpeg` is available on the PATH.
    /// - It uses `reqwest` to download the uploaded object via a presigned URL
    ///   and then re-uploads results with `put_object_from_path`.
    pub async fn process_uploaded_video(&self, bucket: &str, object: &str, user_id: i32, notifications: NotificationsState) -> Result<()> {
        // prepare temp directory
        let tmp_dir = std::env::temp_dir().join(format!("video_process_{}_{}", user_id, Utc::now().timestamp()));
        tokio_fs::create_dir_all(&tmp_dir).await?;

        let src_path = tmp_dir.join("uploaded_input");
        // download
        self.download_via_presigned(bucket, object, src_path.clone()).await?;

        // build output paths
        let processed_video = tmp_dir.join("processed.mp4");
        let extracted_audio = tmp_dir.join("audio.mp3");

        // run ffmpeg to transcode and extract audio. Use tokio::process to avoid blocking.
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
            let _ = notifications.send_notification(user_id, "video:processing_failed", format!("Failed to transcode {}", object));
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
            let _ = notifications.send_notification(user_id, "video:audio_extraction_failed", format!("Failed to extract audio for {}", object));
            return Err(anyhow::anyhow!("ffmpeg audio extraction failed"));
        }

        // upload processed files back to bucket under processed/{object}
        let processed_object = format!("processed/{}", object);
        let audio_object = format!("processed/audio/{}.mp3", object.replace('/', "_"));

        self.put_object_from_path(bucket, &processed_object, processed_video.clone()).await?;
        self.put_object_from_path(bucket, &audio_object, extracted_audio.clone()).await?;

        // notify user
        notifications.send_notification(user_id, "video:processed", format!("Your video '{}' has been processed. Video: {} Audio: {}", object, processed_object, audio_object))?;

        // cleanup
        let _ = tokio_fs::remove_dir_all(&tmp_dir).await;

        Ok(())
    }
}
