use actix_web::{body::to_bytes, http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::rewards::list_candidate_audit::RewardCandidateAuditUseCase;
use rust_learn::db::schema::{courses, reward_audit_events, reward_candidates};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::REWARD_STATUS_PENDING_TEACHER_APPROVAL;
use rust_learn::infra::postgres::rewards::reward_candidate_audit_use_case::PostgresRewardCandidateAuditUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED,
};
use rust_learn::models::reward_candidate::{NewRewardCandidate, RewardCandidate};
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::{json, Value};
use std::sync::Arc;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn reward_candidate_audit_use_case(pool: &DbPool) -> Arc<dyn RewardCandidateAuditUseCase> {
    Arc::new(PostgresRewardCandidateAuditUseCase::new(pool.clone()))
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    create_user(
        conn,
        prefix,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_course(conn: &mut AsyncPgConnection) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("RewardCandidateAuditCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn create_candidate(
    conn: &mut AsyncPgConnection,
    course: &Course,
    student: &User,
    submitter: &User,
) -> RewardCandidate {
    diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id: course.id,
            student_user_id: student.id(),
            submitter_user_id: submitter.id(),
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("candidate-audit"),
            evidence: json!({"completion_percentage": 100}),
            status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create reward candidate")
}
