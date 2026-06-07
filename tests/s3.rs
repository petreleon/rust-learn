use std::env;
use std::sync::atomic::{AtomicU64, Ordering};

use rust_learn::utils::s3_utils::S3State;

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_test_id(prefix: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let seq = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, seq)
}

fn require_s3_external() -> String {
    let mut ext = env::var("S3_EXTERNAL_DOMAIN").ok();
    if let Some(ref v) = ext {
        if v == "localhost" || v == "127.0.0.1" {
            std::env::set_var("S3_EXTERNAL_DOMAIN", "rustfs");
            ext = Some("rustfs".into());
        }
    }
    ext.expect("S3_EXTERNAL_DOMAIN must be set to run these tests reliably, rather than silently skipping them!")
}

#[tokio::test(flavor = "multi_thread")]
async fn presign_external_get_works() {
    dotenvy::dotenv().ok();
    let _ext = require_s3_external();

    let s3 = S3State::new_from_env().await.expect("init s3");
    // Ensure the test bucket exists by uploading a small temporary file.
    let object = format!("{}.txt", unique_test_id("test-object-get"));
    let tmp = std::env::temp_dir().join(format!("{}.txt", unique_test_id("s3_test_get")));
    std::fs::write(&tmp, b"test").expect("write tmp");
    s3.put_object_from_path("test-bucket", &object, tmp.clone())
        .await
        .expect("ensure bucket/upload");
    let _ = std::fs::remove_file(&tmp);

    let url = s3.presign_external_get("test-bucket", &object, 60).await;
    assert!(url.is_ok(), "presign_external_get failed: {:?}", url.err());
}

#[tokio::test(flavor = "multi_thread")]
async fn presign_external_post_form_data_works() {
    dotenvy::dotenv().ok();
    let _ext = require_s3_external();

    let s3 = S3State::new_from_env().await.expect("init s3");
    // Ensure bucket exists by uploading a small object.
    let object = format!("{}.txt", unique_test_id("test-object-post"));
    let tmp = std::env::temp_dir().join(format!("{}.txt", unique_test_id("s3_test_post")));
    std::fs::write(&tmp, b"test").expect("write tmp");
    s3.put_object_from_path("test-bucket", &object, tmp.clone())
        .await
        .expect("ensure bucket/upload");
    let _ = std::fs::remove_file(&tmp);

    let res = s3
        .presign_external_post_form_data("test-bucket", &object, 300)
        .await;
    assert!(res.is_ok(), "presign POST failed: {:?}", res.err());
}
