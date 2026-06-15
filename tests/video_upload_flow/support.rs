pub(crate) use actix_service::Service;
pub(crate) use actix_web::test;
pub(crate) use chrono::{NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::infra::notifications::NotificationsState;
pub(crate) use rust_learn::infra::object_storage::S3State;
pub(crate) use rust_learn::infra::postgres::content::upload_job_queue;
pub(crate) use rust_learn::infra::postgres::models::chapter::Chapter;
pub(crate) use rust_learn::infra::postgres::models::content::Content;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::upload_job::UploadJob;
pub(crate) use rust_learn::infra::postgres::schema::{courses, notifications, upload_jobs};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde::Deserialize;
pub(crate) use std::path::Path;
pub(crate) use std::time::Duration as StdDuration;

use rust_learn::infra::postgres::access_control::{course_role_records, role_catalog_store};
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::models::user::User;

#[derive(Deserialize)]
pub(crate) struct UploadUrlResponse {
    pub(crate) upload_url: String,
    pub(crate) object_key: String,
}

#[derive(Deserialize)]
pub(crate) struct MediaUrlResponse {
    pub(crate) url: String,
}

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
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

pub(crate) async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("assign failed");
}

pub(crate) async fn assert_s3_object_downloadable(s3: &S3State, object: &str) {
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
