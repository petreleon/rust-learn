use actix_service::Service;
use actix_web::{test, web, App};
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{courses, notifications, upload_jobs};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::Chapter;
use rust_learn::models::content::Content;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::role::CourseRole;
use rust_learn::models::upload_job::UploadJob;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::utils::notifications::NotificationsState;
use rust_learn::utils::s3_utils::S3State;
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use std::time::Duration as StdDuration;

#[derive(Deserialize)]
struct UploadUrlResponse {
    upload_url: String,
    object_key: String,
}

fn unique_string(prefix: &str) -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = unique_string(name) + "@example.com";
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password",
    )
    .await
    .expect("failed to create user")
}

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("role not found");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("assign failed");
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

async fn assert_s3_object_downloadable(s3: &S3State, object: &str) {
    let url = s3
        .presign_get("course-materials", object, 60)
        .await
        .expect("processed object should be presignable");
    let response = reqwest::get(url)
        .await
        .expect("processed object should be reachable");
    let status = response.status();
    let bytes = response.bytes().await.expect("response bytes should load");

    assert!(
        status.is_success(),
        "processed object {} should download successfully, got {}",
        object,
        status
    );
    assert!(!bytes.is_empty(), "processed object {} is empty", object);
}
