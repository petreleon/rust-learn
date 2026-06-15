use crate::support::*;
use std::process::Command;

pub(crate) fn create_sample_video_file() -> (std::path::PathBuf, String) {
    let sample_path = std::env::temp_dir().join(format!("{}.mp4", unique_string("sample_video")));
    generate_sample_video(&sample_path);
    let filename = sample_path
        .file_name()
        .expect("sample video path should have filename")
        .to_string_lossy()
        .to_string();
    (sample_path, filename)
}

pub(crate) async fn put_sample_video(sample_path: &Path, upload_url: &str) {
    let sample_bytes = tokio::fs::read(sample_path)
        .await
        .expect("sample video should be readable");
    let upload_response = reqwest::Client::new()
        .put(upload_url)
        .body(sample_bytes)
        .send()
        .await
        .expect("sample video PUT should reach object storage");
    let upload_status = upload_response.status();
    let upload_body = upload_response.text().await.unwrap_or_default();
    assert!(
        upload_status.is_success(),
        "sample video upload failed with {}: {}",
        upload_status,
        upload_body
    );
}

fn generate_sample_video(path: &Path) {
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-hide_banner",
            "-loglevel",
            "error",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=1:size=160x90:rate=10",
            "-f",
            "lavfi",
            "-i",
            "sine=duration=1:frequency=440:sample_rate=44100",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-shortest",
            path.to_string_lossy().as_ref(),
        ])
        .status()
        .expect("ffmpeg must be installed on PATH to run the video upload flow test");

    assert!(status.success(), "ffmpeg failed to generate sample video");
}
