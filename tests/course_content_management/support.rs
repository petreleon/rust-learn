pub(crate) use actix_service::Service;
pub(crate) use actix_web::{test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::{ExpressionMethods, QueryDsl};
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::content::manage_chapter::ChapterUseCases;
pub(crate) use rust_learn::application::content::manage_content_item::ContentItemUseCases;
pub(crate) use rust_learn::application::content::process_upload_job::ContentProcessingUseCase;
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
pub(crate) use rust_learn::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
pub(crate) use rust_learn::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::chapter::{Chapter, NewChapter};
pub(crate) use rust_learn::infra::postgres::models::content::Content;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::schema::{chapters, courses, upload_jobs};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use std::sync::Arc;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
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

pub(crate) fn generate_token(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to generate token")
}

pub(crate) fn chapter_use_cases_data(pool: &DbPool) -> web::Data<Arc<dyn ChapterUseCases>> {
    web::Data::new(Arc::new(PostgresChapterUseCases::new(pool.clone())))
}

pub(crate) fn content_item_use_cases_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn ContentItemUseCases>> {
    web::Data::new(Arc::new(PostgresContentItemUseCases::new(pool.clone())))
}

pub(crate) fn content_processing_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn ContentProcessingUseCase>> {
    web::Data::new(Arc::new(PostgresContentProcessingUseCase::new(
        pool.clone(),
    )))
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
