use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::organizations::list_organization_member_audit::OrganizationMemberAuditUseCase;
use rust_learn::application::organizations::list_organization_members::OrganizationMemberListUseCase;
use rust_learn::db::schema::{organization_member_audit_events, organizations};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::organization_member_audit_event::{
    NewOrganizationMemberAuditEvent, OrganizationMemberAuditEvent,
};
use rust_learn::models::role::OrganizationRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::infra::postgres::organizations::organization_member_audit_use_case::PostgresOrganizationMemberAuditUseCase;
use rust_learn::infra::postgres::organizations::organization_member_list_use_case::PostgresOrganizationMemberListUseCase;
use rust_learn::repositories::delegated_permission_repository::create_delegated_permission;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::Value;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
}

fn organization_member_list_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberListUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberListUseCase::new(
        pool.clone(),
    )))
}

fn organization_member_audit_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberAuditUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberAuditUseCase::new(
        pool.clone(),
    )))
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}
