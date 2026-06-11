impl S3State {
    /// Generate a presigned PUT URL using the external endpoint.
    pub async fn presign_external_put(
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
#[cfg(test)]
mod tests;
