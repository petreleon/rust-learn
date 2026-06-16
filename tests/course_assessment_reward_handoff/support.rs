pub(crate) use actix_web::{http::StatusCode, test, web, App};
pub(crate) use bigdecimal::BigDecimal;
pub(crate) use chrono::{NaiveDate, Utc};
pub(crate) use diesel::prelude::*;
pub(crate) use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub(crate) use rust_learn::application::learning::submit_assessment_attempt::AssessmentSubmissionUseCase;
pub(crate) use rust_learn::domain::rewards::candidate::event_type::REWARD_EVENT_ASSESSMENT_COMPLETION;
pub(crate) use rust_learn::domain::rewards::policy::{
    REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
pub(crate) use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user;
pub(crate) use rust_learn::infra::postgres::learning::assessment_submission_use_case::PostgresAssessmentSubmissionUseCase;
pub(crate) use rust_learn::infra::postgres::models::reward_candidate::RewardCandidate;
pub(crate) use rust_learn::infra::postgres::models::reward_policy::NewRewardPolicy;
pub(crate) use rust_learn::infra::postgres::models::user::User;
pub(crate) use rust_learn::infra::postgres::schema::{
    assessment_questions, assessments, courses, reward_candidates, reward_policies,
};
pub(crate) use rust_learn::infra::postgres::{establish_connection, DbPool};
pub(crate) use serde_json::{json, Value};
pub(crate) use std::sync::Arc;

use rust_learn::infra::postgres::access_control::{course_role_records, role_catalog_store};
use rust_learn::infra::postgres::models::course::{Course, NewCourse};
use rust_learn::infra::tokens::jwt::create_jwt;

pub(crate) fn unique_string(prefix: &str) -> String {
    format!(
        "{}_{}_{}",
        prefix,
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    )
}

pub(crate) async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get().await.expect("failed to get DB connection")
}

pub(crate) async fn create_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_verified_password_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
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

pub(crate) async fn assign_student(conn: &mut AsyncPgConnection, user_id: i32, course_id: i32) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, "STUDENT")
        .await
        .expect("student role should exist");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign student role");
}

pub(crate) async fn create_reward_policy(conn: &mut AsyncPgConnection, course_id: i32) -> i64 {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: REWARD_EVENT_ASSESSMENT_COMPLETION.to_string(),
            version: 1,
            token_amount: BigDecimal::from(10),
            multiplier: BigDecimal::from(1),
            max_payout: Some(BigDecimal::from(100)),
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .returning(reward_policies::id)
        .get_result(conn)
        .await
        .expect("failed to create reward policy")
}

pub(crate) async fn create_assessment(conn: &mut AsyncPgConnection, course_id: i32) -> (i32, i32) {
    let assessment_id = diesel::insert_into(assessments::table)
        .values((
            assessments::course_id.eq(course_id),
            assessments::title.eq("Reward quiz"),
            assessments::passing_score.eq(70),
            assessments::max_attempts.eq(2),
            assessments::published.eq(true),
        ))
        .returning(assessments::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment");
    let question_id = diesel::insert_into(assessment_questions::table)
        .values((
            assessment_questions::assessment_id.eq(assessment_id),
            assessment_questions::text.eq("Question"),
            assessment_questions::question_type.eq("multiple_choice"),
            assessment_questions::correct_answer.eq(Some("yes".to_string())),
            assessment_questions::points.eq(2),
            assessment_questions::order.eq(1),
        ))
        .returning(assessment_questions::id)
        .get_result(conn)
        .await
        .expect("failed to create question");
    (assessment_id, question_id)
}

pub(crate) fn body(question_id: i32) -> Value {
    let mut answers = serde_json::Map::new();
    answers.insert(question_id.to_string(), json!(" yes "));
    json!({ "answers": Value::Object(answers) })
}

pub(crate) fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

pub(crate) fn assessment_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn AssessmentSubmissionUseCase>> {
    web::Data::new(Arc::new(PostgresAssessmentSubmissionUseCase::new(
        pool.clone(),
    )))
}
