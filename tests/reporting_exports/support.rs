pub(crate) use actix_service::Service;
pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::{Duration, NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::schema::{
    courses, courses_organizations, delegated_permissions, external_transactions,
    internal_transactions, organizations, reward_candidates, reward_execution_jobs,
    reward_fraud_blocks, reward_payout_records, reward_policies, reward_wallet_credit_records,
    teacher_applications, transactions, wallets,
};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::domain::access_control::delegation::DELEGATED_SCOPE_PLATFORM;
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TOKEN_CONFIRMED,
};
pub(crate) use rust_learn::domain::rewards::execution::RewardExecutionJobStatus;
pub(crate) use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::domain::teacher_applications::scope::TEACHER_APPLICATION_SCOPE_PLATFORM;
pub(crate) use rust_learn::domain::teacher_applications::status::TEACHER_APPLICATION_STATUS_SUBMITTED;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::infra::postgres::models::delegated_permission::NewDelegatedPermission;
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::NewRewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::reward_fraud_block::NewRewardFraudBlock;
pub(crate) use rust_learn::infra::postgres::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::infra::postgres::models::teacher_application::NewTeacherApplication;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::models::wallet::NewWallet;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use serde_json::{json, Value};

use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_test_user(conn: &mut AsyncPgConnection, name: &str) -> User {
    let email = format!("{}@example.com", unique_string(name));
    create_user(
        conn,
        name,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
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

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: unique_string("ReportingOrg"),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

pub(crate) async fn create_course(conn: &mut AsyncPgConnection) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("ReportingCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}
