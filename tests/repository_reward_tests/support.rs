pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::{NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::db::establish_connection;
pub(crate) use rust_learn::db::schema::{courses, users};
pub(crate) use rust_learn::domain::rewards::audit::REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED;
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
pub(crate) use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
pub(crate) use rust_learn::domain::rewards::candidate::status::{
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
};
pub(crate) use rust_learn::domain::rewards::execution::REWARD_EXECUTION_STATUS_QUEUED;
pub(crate) use rust_learn::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
pub(crate) use rust_learn::infra::postgres::models::course::{Course, NewCourse};
pub(crate) use rust_learn::infra::postgres::models::reward_audit_event::NewRewardAuditEvent;
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::NewRewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::reward_fraud_block::NewRewardFraudBlock;
pub(crate) use rust_learn::infra::postgres::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::rewards::reward_audit_records as reward_audit_event_repository;
pub(crate) use rust_learn::infra::postgres::rewards::reward_candidate_records::{
    self as reward_candidate_repository, RewardCandidateFilter,
};
pub(crate) use rust_learn::infra::postgres::rewards::reward_execution_job_records as reward_execution_job_repository;
pub(crate) use rust_learn::infra::postgres::rewards::reward_fraud_block_records::{
    self as reward_fraud_block_repository, RewardFraudBlockFilter,
};
pub(crate) use serde_json::json;

pub(crate) mod reward_policy_repository {
    pub use rust_learn::infra::postgres::rewards::reward_policy_activation::deactivate_active_policies;
    pub use rust_learn::infra::postgres::rewards::reward_policy_records::{
        create_policy, list_policies, next_policy_version, RewardPolicyFilter,
    };
}

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

pub(crate) fn new_candidate(course_id: i32, student_user_id: i32, key: &str) -> NewRewardCandidate {
    NewRewardCandidate {
        course_id,
        student_user_id,
        submitter_user_id: student_user_id,
        source_scope: REWARD_SOURCE_COURSE.to_string(),
        source_organization_id: None,
        event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
        idempotency_key: key.to_string(),
        evidence: json!({"completion_percentage": 100.0}),
        status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
    }
}
