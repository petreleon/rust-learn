use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::identity::current_session::CurrentSessionUseCase;
use rust_learn::application::notifications::notification_inbox::NotificationInboxUseCase;
use rust_learn::application::notifications::preference_service::NotificationPreferencesUseCase;
use rust_learn::db::schema::{courses, organizations, users};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
use rust_learn::infra::postgres::notifications::notification_inbox_use_case::PostgresNotificationInboxUseCase;
use rust_learn::infra::postgres::notifications::notification_preferences_use_case::PostgresNotificationPreferencesUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
use rust_learn::infra::postgres::access_control::organization_role_records;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::delegated_permission_repository::create_delegated_permission;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::utils::notifications::NotificationsState;
use serde_json::Value;
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn notification_preferences_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn NotificationPreferencesUseCase>> {
    web::Data::new(Arc::new(PostgresNotificationPreferencesUseCase::new(
        pool.clone(),
    )))
}

fn current_session_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn CurrentSessionUseCase>> {
    web::Data::new(Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())))
}

fn notification_inbox_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn NotificationInboxUseCase>> {
    web::Data::new(Arc::new(PostgresNotificationInboxUseCase::new(
        pool.clone(),
    )))
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

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
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
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}
