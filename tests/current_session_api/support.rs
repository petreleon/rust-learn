pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::identity::current_session::CurrentSessionUseCase;
pub(crate) use rust_learn::application::notifications::notification_inbox::NotificationInboxUseCase;
pub(crate) use rust_learn::application::notifications::preference_service::NotificationPreferencesUseCase;
pub(crate) use rust_learn::infra::notifications::NotificationsState;
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::delegated_permissions::create_delegated_permission;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::delegated_permission::NewDelegatedPermission;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::notifications::notification_inbox_use_case::PostgresNotificationInboxUseCase;
pub(crate) use rust_learn::infra::postgres::notifications::notification_preferences_use_case::PostgresNotificationPreferencesUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{courses, organizations, users};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::Value;
pub(crate) use std::sync::Arc;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

pub(crate) fn notification_preferences_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn NotificationPreferencesUseCase>> {
    web::Data::new(Arc::new(PostgresNotificationPreferencesUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn current_session_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn CurrentSessionUseCase>> {
    web::Data::new(Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())))
}

pub(crate) fn notification_inbox_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn NotificationInboxUseCase>> {
    web::Data::new(Arc::new(PostgresNotificationInboxUseCase::new(
        pool.clone(),
    )))
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
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

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
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

pub(crate) async fn assign_platform_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role should exist");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

pub(crate) async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::organization_role_id_by_name(conn, role_name)
        .await
        .expect("organization role should exist");
    organization_role_records::assign_organization_role_to_user(
        conn,
        user_id,
        organization_id,
        role_id,
    )
    .await
    .expect("failed to assign organization role");
}

pub(crate) async fn assign_course_role(
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

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}
