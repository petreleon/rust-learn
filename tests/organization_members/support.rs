pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::organizations::assign_organization_member_role::OrganizationMemberRoleAssignmentUseCase;
pub(crate) use rust_learn::application::organizations::invite_organization_member::OrganizationMemberInviteUseCase;
pub(crate) use rust_learn::application::organizations::list_organization_member_audit::OrganizationMemberAuditUseCase;
pub(crate) use rust_learn::application::organizations::list_organization_members::OrganizationMemberListUseCase;
pub(crate) use rust_learn::application::organizations::remove_organization_member::OrganizationMemberRemovalUseCase;
pub(crate) use rust_learn::db::schema::{
    organization_member_audit_events, organizations, user_role_organization,
};
pub(crate) use rust_learn::db::{DbPool, establish_connection};
pub(crate) use rust_learn::infra::postgres::access_control::delegated_permissions::create_delegated_permission;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::organizations::organization_member_audit_use_case::PostgresOrganizationMemberAuditUseCase;
pub(crate) use rust_learn::infra::postgres::organizations::organization_member_invite_use_case::PostgresOrganizationMemberInviteUseCase;
pub(crate) use rust_learn::infra::postgres::organizations::organization_member_list_use_case::PostgresOrganizationMemberListUseCase;
pub(crate) use rust_learn::infra::postgres::organizations::organization_member_removal_use_case::PostgresOrganizationMemberRemovalUseCase;
pub(crate) use rust_learn::infra::postgres::organizations::organization_member_role_assignment_use_case::PostgresOrganizationMemberRoleAssignmentUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use rust_learn::infra::postgres::models::delegated_permission::NewDelegatedPermission;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::organization_member_audit_event::{
    NewOrganizationMemberAuditEvent, OrganizationMemberAuditEvent,
};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use serde_json::Value;
pub(crate) use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

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

pub(crate) fn organization_member_list_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberListUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberListUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn organization_member_audit_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberAuditUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberAuditUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn organization_member_invite_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberInviteUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberInviteUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn organization_member_removal_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberRemovalUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationMemberRemovalUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn organization_member_role_assignment_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationMemberRoleAssignmentUseCase>> {
    web::Data::new(Arc::new(
        PostgresOrganizationMemberRoleAssignmentUseCase::new(pool.clone()),
    ))
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}

pub(crate) async fn member_role_count(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    user_id: i32,
) -> i64 {
    user_role_organization::table
        .filter(user_role_organization::organization_id.eq(Some(organization_id)))
        .filter(user_role_organization::user_id.eq(Some(user_id)))
        .count()
        .get_result(conn)
        .await
        .expect("failed to count organization member roles")
}
