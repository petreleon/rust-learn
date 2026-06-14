use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::establish_connection;
use rust_learn::db::schema::{courses, reward_candidates, transactions, wallets};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::REWARD_STATUS_COMPLETED;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::platform_role_records;
use rust_learn::repositories::reward_candidate_repository::find_candidate;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::application::rewards::record_compensation::{
    RecordRewardCompensationCommand as RewardCompensationRequest, RewardCompensationError,
    RewardCompensationOutput, RewardCompensationUseCase,
};
use rust_learn::domain::rewards::compensation::REWARD_TRANSACTION_TYPE_COMPENSATION;
use rust_learn::infra::postgres::rewards::reward_compensation_use_case::PostgresRewardCompensationUseCase;
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

async fn record_reward_compensation(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: RewardCompensationRequest,
) -> Result<RewardCompensationOutput, RewardCompensationError> {
    let pool = establish_connection();
    PostgresRewardCompensationUseCase::new(pool)
        .record_reward_compensation(actor_user_id, request)
        .await
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

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role not found");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_completed_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
) -> i64 {
    let candidate_id: i64 = diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("reward_compensation_candidate"),
            evidence: json!({ "completion_percentage": 100 }),
            status: REWARD_STATUS_COMPLETED.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate");

    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::approved_amount.eq(Some(BigDecimal::from(10))),
            reward_candidates::amount_reviewer_user_id.eq(Some(submitter_user_id)),
            reward_candidates::amount_decided_at.eq(Some(chrono::Utc::now())),
        ))
        .execute(conn)
        .await
        .expect("failed to mark reward candidate completed");

    candidate_id
}
