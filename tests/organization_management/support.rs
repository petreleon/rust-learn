pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::organizations::manage_organizations::OrganizationManagementUseCase;
pub(crate) use rust_learn::config::constants::{permissions::Permissions, roles::Roles};
pub(crate) use rust_learn::db::schema::{
    courses, courses_organizations, delegated_permissions, organizations,
};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::domain::access_control::delegation::DELEGATED_SCOPE_ORGANIZATION;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::delegated_permission::NewDelegatedPermission;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::organizations::organization_management_use_case::PostgresOrganizationManagementUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::Value;
pub(crate) use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn organization_management_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationManagementUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationManagementUseCase::new(
        pool.clone(),
    )))
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get().await.expect("failed to get DB connection")
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

pub(crate) async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: format!("{} {}", title, unique_string("course")),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
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

pub(crate) async fn delegate_manage_org_settings(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(delegated_permissions::table)
        .values(NewDelegatedPermission {
            grantor_user_id: user_id,
            grantee_user_id: user_id,
            permission: Permissions::MANAGE_ORG_SETTINGS.to_string(),
            scope_type: DELEGATED_SCOPE_ORGANIZATION.to_string(),
            organization_id: Some(organization_id),
            course_id: None,
            reason: Some("organization management route test".to_string()),
            expires_at: None,
        })
        .execute(conn)
        .await
        .expect("failed to delegate organization settings permission");
}

pub(crate) async fn course_link_order(
    pool: &DbPool,
    organization_id: i32,
    course_id: i32,
) -> Option<i32> {
    let mut conn = setup_conn(pool).await;
    courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::order)
        .first::<i32>(&mut conn)
        .await
        .optional()
        .expect("failed to load course organization link")
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}
