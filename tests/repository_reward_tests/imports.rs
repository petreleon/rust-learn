use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, users};
use rust_learn::domain::rewards::audit::REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED;
use rust_learn::domain::rewards::execution::REWARD_EXECUTION_STATUS_QUEUED;
use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
};
use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_audit_event::NewRewardAuditEvent;
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::models::reward_fraud_block::NewRewardFraudBlock;
use rust_learn::models::reward_policy::NewRewardPolicy;
use rust_learn::models::user::User;
use rust_learn::repositories::reward_audit_event_repository;
use rust_learn::repositories::reward_candidate_repository::{self, RewardCandidateFilter};
use rust_learn::repositories::reward_execution_job_repository;
use rust_learn::repositories::reward_fraud_block_repository::{self, RewardFraudBlockFilter};
use rust_learn::repositories::reward_policy_repository;
use rust_learn::repositories::user_repository::create_user;
use serde_json::json;

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
