use std::sync::Arc;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

use minio::s3::{Client, client::ClientBuilder, creds::StaticProvider, builders::{ObjectContent, GetPresignedObjectUrl, PostPolicy, GetPresignedPolicyFormData}};
use minio::s3::types::S3Api;
use std::collections::HashMap;
use http::Method;

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
        // The minio-rs v0.3 client exposes a `get_presigned_object_url` builder.
        // Map the expires_seconds into the builder's expiry_seconds (u32) if
        // provided. The builder defaults to 7 days.
        let mut builder = self.0.get_presigned_object_url(bucket, object, Method::GET);
        if expires_seconds > 0 {
            // clamp into u32 and set on builder
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
}
