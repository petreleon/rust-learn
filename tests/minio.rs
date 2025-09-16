use std::env;

use rust_learn::utils::minio_utils::MinioState;

fn normalize_minio_external() -> Option<String> {
    let mut ext = env::var("MINIO_EXTERNAL_DOMAIN").ok();
    if let Some(ref v) = ext {
        if v == "localhost" || v == "127.0.0.1" {
            std::env::set_var("MINIO_EXTERNAL_DOMAIN", "minio");
            ext = Some("minio".into());
        }
    }
    ext
}

#[tokio::test(flavor = "multi_thread")]
async fn presign_external_get_works_or_skips() {
    dotenvy::dotenv().ok();
    let ext = normalize_minio_external();
    if ext.is_none() {
        eprintln!("MINIO_EXTERNAL_DOMAIN not set; skipping test");
        return;
    }

    let minio = MinioState::new_from_env().await.expect("init minio");
    // Ensure the test bucket exists by uploading a small temporary file.
    let tmp = std::env::temp_dir().join(format!("minio_test_{}.txt",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::write(&tmp, b"test").expect("write tmp");
    minio.put_object_from_path("test-bucket", "test-object.txt", tmp.clone()).await.expect("ensure bucket/upload");
    let _ = std::fs::remove_file(&tmp);

    let url = minio.presign_external_get("test-bucket", "test-object.txt", 60).await;
    assert!(url.is_ok(), "presign_external_get failed: {:?}", url.err());
}

#[tokio::test(flavor = "multi_thread")]
async fn presign_external_post_form_data_works_or_skips() {
    dotenvy::dotenv().ok();
    let ext = normalize_minio_external();
    if ext.is_none() {
        eprintln!("MINIO_EXTERNAL_DOMAIN not set; skipping test");
        return;
    }

    // Build a minimal PostPolicy programmatically (use minio types)
    use minio::s3::builders::PostPolicy;
    use minio::s3::utils::utc_now;

    let expiration = utc_now() + chrono::Duration::seconds(300);
    let mut policy = PostPolicy::new("test-bucket", expiration).expect("policy");
    policy.add_equals_condition("key", "test-object.txt").expect("add cond");

    let minio = MinioState::new_from_env().await.expect("init minio");
    // Ensure bucket exists by uploading a small object.
    let tmp = std::env::temp_dir().join(format!("minio_test_{}.txt",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::write(&tmp, b"test").expect("write tmp");
    minio.put_object_from_path("test-bucket", "test-object.txt", tmp.clone()).await.expect("ensure bucket/upload");
    let _ = std::fs::remove_file(&tmp);

    let res = minio.presign_external_post_form_data(policy).await;
    assert!(res.is_ok(), "presign POST failed: {:?}", res.err());
}
