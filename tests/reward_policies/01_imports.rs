use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::config::constants::roles::Roles;
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, organizations};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_candidate::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::models::reward_policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
    REWARD_POLICY_SCOPE_PLATFORM,
};
use rust_learn::models::user::User;
use rust_learn::repositories::platform_repository::assign_role_to_user;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::services::reward_policy_service::{
    create_reward_policy, list_reward_policies, CreateRewardPolicyRequest,
    ListRewardPoliciesRequest, RewardPolicyError,
};
use std::str::FromStr;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}", prefix, ts)
}

async fn setup_conn(
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_user_helper(conn: &mut AsyncPgConnection, prefix: &str) -> User {
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

fn platform_policy_request(amount: &str) -> CreateRewardPolicyRequest {
    CreateRewardPolicyRequest {
        scope_type: REWARD_POLICY_SCOPE_PLATFORM.to_string(),
        organization_id: None,
        course_id: None,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        token_amount: BigDecimal::from_str(amount).expect("valid amount"),
        multiplier: Some(BigDecimal::from(1)),
        max_payout: Some(BigDecimal::from(100)),
        cooldown_seconds: Some(86_400),
        payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
        active: Some(true),
    }
}
