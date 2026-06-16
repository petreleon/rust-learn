pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::NaiveDate;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::rewards::manage_reward_policy::{
    CreateRewardPolicyCommand, ListRewardPoliciesQuery, RewardPolicyError, RewardPolicyUseCase,
    UpdateRewardPolicyActivationCommand,
};
pub(crate) use rust_learn::domain::access_control::roles::Roles;
pub(crate) use rust_learn::domain::rewards::policy::{
    RewardPaymentStrategy, RewardPolicyAuditEventType, RewardPolicyEventType, RewardPolicyScope,
};
pub(crate) use rust_learn::infra::postgres::access_control::role_assignments::assign_platform_role_to_user;
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::organization::{NewOrganization, Organization};
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::rewards::reward_policy_use_case::PostgresRewardPolicyUseCase;
pub(crate) use rust_learn::infra::postgres::schema::{courses, organizations};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use std::str::FromStr;

pub(crate) fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

pub(crate) fn setup_pool() -> DbPool {
    let _ = dotenvy::dotenv();
    establish_connection()
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

pub(crate) async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user")
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

pub(crate) fn reward_policy_use_case(pool: &DbPool) -> PostgresRewardPolicyUseCase {
    PostgresRewardPolicyUseCase::new(pool.clone())
}

pub(crate) fn platform_policy_request(amount: &str) -> CreateRewardPolicyCommand {
    CreateRewardPolicyCommand {
        scope_type: RewardPolicyScope::Platform,
        organization_id: None,
        course_id: None,
        event_type: RewardPolicyEventType::CourseCompletion,
        token_amount: BigDecimal::from_str(amount).expect("valid amount"),
        multiplier: Some(BigDecimal::from(1)),
        max_payout: Some(BigDecimal::from(100)),
        cooldown_seconds: Some(86_400),
        payment_strategy: RewardPaymentStrategy::TreasuryTransfer,
        active: Some(true),
    }
}
