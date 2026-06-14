use actix_web::{body::to_bytes, http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::rewards::manage_fraud_block::{
    CreateRewardFraudBlockCommand, RewardFraudBlockError, RewardFraudBlockOutput,
    RewardFraudBlockUseCase,
};
use rust_learn::application::rewards::list_platform_candidates::PlatformRewardCandidatesUseCase;
use rust_learn::config::constants::permissions::Permissions;
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::db::schema::{
    course_roles, courses, courses_organizations, organizations, platform_roles,
    reward_execution_jobs, reward_policies, role_permission_course, role_permission_platform,
    users,
};
use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::domain::rewards::candidate::event_type::{
    REWARD_EVENT_COURSE_COMPLETION, REWARD_EVENT_MANUAL_COMPLETION,
};
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_ORGANIZATION;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::domain::rewards::execution::REWARD_EXECUTION_STATUS_QUEUED;
use rust_learn::infra::postgres::rewards::platform_reward_candidate_use_case::PostgresPlatformRewardCandidatesUseCase;
use rust_learn::infra::postgres::rewards::reward_fraud_block_use_case::PostgresRewardFraudBlockUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_audit_event::{
    REWARD_AUDIT_EVENT_AMOUNT_DECISION, REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED,
    REWARD_AUDIT_EVENT_TEACHER_DECISION,
};
use rust_learn::models::reward_policy::NewRewardPolicy;
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::reward_audit_event_repository::list_reward_audit_events;
use rust_learn::repositories::reward_candidate_repository::find_candidate;
use rust_learn::repositories::reward_execution_job_repository::find_job_by_candidate;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_candidate_service::{
    decide_reward_candidate_by_teacher, RewardCandidateError, TeacherRewardCandidateDecisionRequest,
};
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};
use std::sync::Arc;
use std::str::FromStr;

type RewardFraudBlockRequest = CreateRewardFraudBlockCommand;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn platform_reward_candidates_use_case(
    pool: &DbPool,
) -> Arc<dyn PlatformRewardCandidatesUseCase> {
    Arc::new(PostgresPlatformRewardCandidatesUseCase::new(pool.clone()))
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_reward_fraud_block(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: RewardFraudBlockRequest,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let pool = establish_connection();
    PostgresRewardFraudBlockUseCase::new(pool)
        .create_reward_fraud_block(actor_user_id, request)
        .await
}

async fn revoke_reward_fraud_block(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    block_id: i64,
) -> Result<RewardFraudBlockOutput, RewardFraudBlockError> {
    let pool = establish_connection();
    PostgresRewardFraudBlockUseCase::new(pool)
        .revoke_reward_fraud_block(actor_user_id, block_id)
        .await
}

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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
