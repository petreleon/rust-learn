use actix_service::Service;
use actix_web::{test, web, App};
use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{chapters, courses, upload_jobs};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::{Chapter, NewChapter};
use rust_learn::models::content::Content;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::application::content::manage_chapter::ChapterUseCases;
use rust_learn::application::content::manage_content_item::ContentItemUseCases;
use rust_learn::application::content::process_upload_job::ContentProcessingUseCase;
use rust_learn::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use rust_learn::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use rust_learn::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use rust_learn::utils::jwt_utils::create_jwt;
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
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

fn generate_token(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to generate token")
}

fn chapter_use_cases_data(pool: &DbPool) -> web::Data<Arc<dyn ChapterUseCases>> {
    web::Data::new(Arc::new(PostgresChapterUseCases::new(pool.clone())))
}

fn content_item_use_cases_data(pool: &DbPool) -> web::Data<Arc<dyn ContentItemUseCases>> {
    web::Data::new(Arc::new(PostgresContentItemUseCases::new(pool.clone())))
}

fn content_processing_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn ContentProcessingUseCase>> {
    web::Data::new(Arc::new(PostgresContentProcessingUseCase::new(
        pool.clone(),
    )))
}

async fn force_assign_course_role(
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
