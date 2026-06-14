use std::sync::Arc;

use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::assign_course_role::CourseRoleAssignmentUseCase;
use rust_learn::application::learning::course_enrollment::CourseEnrollmentUseCase;
use rust_learn::db::schema::{courses, notifications};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::learning::enrollment::status::COURSE_JOIN_STATUS_APPROVED;
use rust_learn::infra::postgres::learning::course_enrollment_use_case::PostgresCourseEnrollmentUseCase;
use rust_learn::infra::postgres::learning::course_role_assignment_use_case::PostgresCourseRoleAssignmentUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::utils::notifications::NotificationsState;
use serde_json::Value;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn course_enrollment_test_app(
    pool: DbPool,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(course_enrollment_use_case_data(&pool))
        .app_data(course_role_assignment_use_case_data(&pool))
        .app_data(web::Data::new(NotificationsState::new(pool)))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(rust_learn::http::learning::course_scope())
}

fn course_enrollment_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn CourseEnrollmentUseCase>> {
    web::Data::new(Arc::new(PostgresCourseEnrollmentUseCase::new(pool.clone())))
}

fn course_role_assignment_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CourseRoleAssignmentUseCase>> {
    web::Data::new(Arc::new(PostgresCourseRoleAssignmentUseCase::new(pool.clone())))
}

async fn notification_count(conn: &mut AsyncPgConnection, user_id: i32, title: &str) -> i64 {
    notifications::table
        .filter(notifications::user_id.eq(Some(user_id)))
        .filter(notifications::title.eq(title))
        .count()
        .get_result(conn)
        .await
        .expect("notification count should load")
}
