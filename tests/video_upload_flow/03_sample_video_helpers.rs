fn create_sample_video_file() -> (std::path::PathBuf, String) {
    let sample_path = std::env::temp_dir().join(format!("{}.mp4", unique_string("sample_video")));
    generate_sample_video(&sample_path);
    let filename = sample_path
        .file_name()
        .expect("sample video path should have filename")
        .to_string_lossy()
        .to_string();
    (sample_path, filename)
}

async fn put_sample_video(sample_path: &Path, upload_url: &str) {
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
