use crate::reward_candidate_error::RewardCandidateError;

pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionOutput, DelegatedPermissionUseCase,
    GrantDelegatedPermissionCommand, RevokeDelegatedPermissionCommand,
};
pub(crate) use rust_learn::application::rewards::decide_amount::{
    RewardAmountDecisionCommand as RewardAmountDecisionRequest, RewardAmountDecisionError,
    RewardAmountDecisionOutput, RewardAmountDecisionUseCase,
};
pub(crate) use rust_learn::config::constants::permissions::Permissions;
pub(crate) use rust_learn::db::establish_connection;
pub(crate) use rust_learn::db::schema::{
    courses, courses_organizations, delegated_permissions, organizations, reward_policies, users,
};
pub(crate) use rust_learn::domain::access_control::delegation::{
    DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::access_control::course_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::delegated_permissions::{
    find_delegated_permission, use_case::PostgresDelegatedPermissionUseCase,
};
pub(crate) use rust_learn::infra::postgres::access_control::permission_queries::{
    has_course_permission, has_organization_permission, has_platform_permission,
};
pub(crate) use rust_learn::infra::postgres::access_control::platform_role_records;
pub(crate) use rust_learn::infra::postgres::access_control::role_catalog_store;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::rewards::reward_amount_decision_use_case::PostgresRewardAmountDecisionUseCase;
pub(crate) use rust_learn::models::course::{Course, NewCourse};
pub(crate) use rust_learn::models::courses_organizations::NewCourseOrganization;
pub(crate) use rust_learn::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::models::user::User;
pub(crate) use serde_json::json;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn decide_reward_amount(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    candidate_id: i64,
    request: RewardAmountDecisionRequest,
) -> Result<RewardAmountDecisionOutput, RewardCandidateError> {
    let pool = establish_connection();
    PostgresRewardAmountDecisionUseCase::new(pool)
        .decide_reward_amount(actor_user_id, candidate_id, request)
        .await
        .map_err(map_reward_amount_decision_error)
}

fn map_reward_amount_decision_error(error: RewardAmountDecisionError) -> RewardCandidateError {
    match error {
        RewardAmountDecisionError::PermissionDenied(permission) => {
            RewardCandidateError::PermissionDenied(permission)
        }
        RewardAmountDecisionError::InvalidInput(message) => {
            RewardCandidateError::InvalidInput(message)
        }
        RewardAmountDecisionError::InvalidStatus(message) => {
            RewardCandidateError::InvalidStatus(message)
        }
        RewardAmountDecisionError::NotFound => RewardCandidateError::NotFound,
        RewardAmountDecisionError::Connection(message)
        | RewardAmountDecisionError::Database(message) => RewardCandidateError::Database(message),
    }
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

pub(crate) async fn link_course_to_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}
