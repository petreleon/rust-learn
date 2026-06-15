pub(crate) use actix_web::{body::to_bytes, http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::rewards::list_platform_candidates::PlatformRewardCandidatesUseCase;
pub(crate) use rust_learn::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, RewardFraudBlockError, RewardFraudBlockOutput,
    RewardFraudBlockUseCase,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::schema::{
    course_roles, courses, courses_organizations, organizations, platform_roles,
    reward_execution_jobs, reward_policies, role_permission_course, role_permission_platform,
    users,
};
pub(crate) use rust_learn::db::{establish_connection, DbPool};
pub(crate) use rust_learn::domain::rewards::audit::{
    REWARD_AUDIT_EVENT_AMOUNT_DECISION, REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED,
    REWARD_AUDIT_EVENT_TEACHER_DECISION,
};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::{
    RewardEventType, REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED,
};
pub(crate) use rust_learn::domain::rewards::execution::REWARD_EXECUTION_STATUS_QUEUED;
pub(crate) use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::organization_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::rewards::platform_reward_candidate_use_case::PostgresPlatformRewardCandidatesUseCase;
pub(crate) use rust_learn::infra::postgres::rewards::reward_audit_records::list_reward_audit_events;
pub(crate) use rust_learn::infra::postgres::rewards::reward_candidate_records::find_candidate;
pub(crate) use rust_learn::infra::postgres::rewards::reward_execution_job_records::find_job_by_candidate;
pub(crate) use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
pub(crate) use rust_learn::infra::tokens::jwt::create_jwt;
pub(crate) use rust_learn::models::course::{Course, NewCourse};
pub(crate) use rust_learn::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::models::user::User;
pub(crate) use serde_json::{json, Value};

pub(crate) use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
pub(crate) use std::sync::Arc;

pub(crate) type RewardFraudBlockRequest = CreateRewardFraudBlockCommand;

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}", prefix, ts, counter)
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn platform_reward_candidates_use_case(
    pool: &DbPool,
) -> Arc<dyn PlatformRewardCandidatesUseCase> {
    Arc::new(PostgresPlatformRewardCandidatesUseCase::new(pool.clone()))
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_reward_fraud_block(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: RewardFraudBlockRequest,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let pool = establish_connection();
    PostgresRewardFraudBlockUseCase::new(pool)
        .create_reward_fraud_block(actor_user_id, request)
        .await
}

pub(crate) async fn revoke_reward_fraud_block(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    block_id: i64,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let pool = establish_connection();
    PostgresRewardFraudBlockUseCase::new(pool)
        .revoke_reward_fraud_block(actor_user_id, block_id)
        .await
}

pub(crate) async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let user = create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(conn)
        .await
        .expect("failed to verify user email");
    user
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
