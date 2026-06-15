pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::organizations::list_organization_teacher_applications::OrganizationTeacherApplicationListUseCase;
pub(crate) use rust_learn::application::teacher_applications::TeacherApplicationOutput;
pub(crate) use rust_learn::config::constants::roles::Roles;
pub(crate) use rust_learn::db::schema::organizations;
pub(crate) use rust_learn::db::{DbPool, establish_connection};
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::organizations::organization_teacher_application_use_case::PostgresOrganizationTeacherApplicationUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use rust_learn::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::models::user::User;
pub(crate) use serde_json::Value;
pub(crate) use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TeacherApplicationError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidTransition(String),
    NotFound,
    Database(String),
}

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    create_user(
        conn,
        &format!("{} User", prefix.replace('_', " ")),
        &email,
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

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn organization_teacher_application_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationTeacherApplicationListUseCase>> {
    web::Data::new(Arc::new(
        PostgresOrganizationTeacherApplicationUseCase::new(pool.clone()),
    ))
}
