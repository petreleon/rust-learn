use anyhow::Result;
use chrono::Utc;
use std::process::Stdio;

use tokio::fs as tokio_fs;
use tokio::process::Command as TokioCommand;

use crate::infra::notifications::NotificationsState;

use super::state::S3State;

impl S3State {
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
