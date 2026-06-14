use actix_web::{body::to_bytes, http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::rewards::list_course_candidates::CourseRewardCandidatesUseCase;
use rust_learn::db::schema::{courses, reward_candidates, users};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_COURSE_COMPLETION;
use rust_learn::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use rust_learn::domain::rewards::candidate::status::REWARD_STATUS_PENDING_TEACHER_APPROVAL;
use rust_learn::infra::postgres::rewards::course_reward_candidate_use_case::PostgresCourseRewardCandidatesUseCase;
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::reward_candidate::NewRewardCandidate;
use rust_learn::infra::postgres::access_control::role_catalog_store;
use rust_learn::models::user::User;
use rust_learn::infra::postgres::access_control::course_role_records;
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

fn course_reward_candidates_use_case(pool: &DbPool) -> Arc<dyn CourseRewardCandidatesUseCase> {
    Arc::new(PostgresCourseRewardCandidatesUseCase::new(pool.clone()))
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_verified_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    let email = format!("{}@example.com", unique_string(prefix));
    let user = create_user(
        conn,
        prefix,
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
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

async fn create_course(conn: &mut AsyncPgConnection) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: unique_string("RewardCourseCandidatesCourse"),
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn create_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    submitter_user_id: i32,
) -> i64 {
    diesel::insert_into(reward_candidates::table)
        .values(NewRewardCandidate {
            course_id,
            student_user_id,
            submitter_user_id,
            source_scope: REWARD_SOURCE_COURSE.to_string(),
            source_organization_id: None,
            event_type: REWARD_EVENT_COURSE_COMPLETION.to_string(),
            idempotency_key: unique_string("course-candidate-list"),
            evidence: json!({"completion_percentage": 100}),
            status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
        })
        .returning(reward_candidates::id)
        .get_result(conn)
        .await
        .expect("failed to create reward candidate")
}
