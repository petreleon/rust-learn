use std::sync::Arc;

use actix_web::{http::StatusCode, test, web, App};
use chrono::{Duration, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::list_assessment_attempts::AssessmentAttemptsUseCase;
use rust_learn::application::learning::list_course_assessments::CourseAssessmentsUseCase;
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user as create_user;
use rust_learn::infra::postgres::learning::assessment_read_use_case::PostgresAssessmentReadUseCase;
use rust_learn::infra::postgres::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::models::user::User;
use rust_learn::infra::postgres::schema::{
    assessment_attempts, assessment_questions, assessments, courses,
};
use rust_learn::infra::postgres::{establish_connection, DbPool};
use rust_learn::infra::tokens::jwt::create_jwt;
use serde_json::{json, Value};

fn unique_string(prefix: &str) -> String {
    let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}_{}_{}", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection, prefix: &str) -> User {
    create_user(
        conn,
        &format!("{} Test", prefix),
        &(unique_string(prefix) + "@example.com"),
        Some(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
        "ValidPass123!",
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

async fn create_assessment(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    title: &str,
    published: bool,
) -> i32 {
    diesel::insert_into(assessments::table)
        .values((
            assessments::course_id.eq(course_id),
            assessments::title.eq(title),
            assessments::description.eq(Some(format!("{} description", title))),
            assessments::passing_score.eq(70),
            assessments::max_attempts.eq(3),
            assessments::published.eq(published),
        ))
        .returning(assessments::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment")
}

async fn create_attempt(
    conn: &mut AsyncPgConnection,
    assessment_id: i32,
    user_id: i32,
    minutes_ago: i64,
) -> i32 {
    let completed_at = Utc::now() - Duration::minutes(minutes_ago);

    diesel::insert_into(assessment_attempts::table)
        .values((
            assessment_attempts::assessment_id.eq(assessment_id),
            assessment_attempts::user_id.eq(user_id),
            assessment_attempts::score.eq(10),
            assessment_attempts::passed.eq(true),
            assessment_attempts::started_at.eq(completed_at),
            assessment_attempts::completed_at.eq(Some(completed_at)),
        ))
        .returning(assessment_attempts::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment attempt")
}

async fn create_question(conn: &mut AsyncPgConnection, assessment_id: i32) -> i32 {
    diesel::insert_into(assessment_questions::table)
        .values((
            assessment_questions::assessment_id.eq(assessment_id),
            assessment_questions::text.eq("Which Rust concept prevents aliasing bugs?"),
            assessment_questions::question_type.eq("multiple_choice"),
            assessment_questions::options.eq(Some(json!(["Ownership", "Prototype chains"]))),
            assessment_questions::correct_answer.eq(Some("Ownership".to_string())),
            assessment_questions::points.eq(2),
            assessment_questions::order.eq(0),
        ))
        .returning(assessment_questions::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment question")
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn course_assessments_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn CourseAssessmentsUseCase>> {
    web::Data::new(Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())))
}

fn assessment_attempts_use_case_data(
    pool: &DbPool,
) -> web::Data<Arc<dyn AssessmentAttemptsUseCase>> {
    web::Data::new(Arc::new(PostgresAssessmentReadUseCase::new(pool.clone())))
}

#[actix_web::test]
async fn assessment_read_routes_are_published_and_user_scoped() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_test_user(&mut conn, "assessment_learner").await;
    let other_user = create_test_user(&mut conn, "assessment_other").await;
    let course = create_course(&mut conn, &unique_string("AssessmentCourse")).await;
    let published_id = create_assessment(&mut conn, course.id, "Published quiz", true).await;
    let draft_id = create_assessment(&mut conn, course.id, "Draft quiz", false).await;
    let question_id = create_question(&mut conn, published_id).await;
    let older_attempt_id = create_attempt(&mut conn, published_id, learner.id(), 30).await;
    let newer_attempt_id = create_attempt(&mut conn, published_id, learner.id(), 10).await;
    create_attempt(&mut conn, published_id, other_user.id(), 5).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(course_assessments_use_case_data(&pool))
            .app_data(assessment_attempts_use_case_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let list_req = test::TestRequest::get()
        .uri(&format!("/courses/{}/assessments", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let list_response = test::call_service(&app, list_req).await;
    assert_eq!(list_response.status(), StatusCode::OK);
    let assessments_json: Value = test::read_body_json(list_response).await;
    let listed = assessments_json
        .as_array()
        .expect("assessment list response");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["id"], published_id);
    assert_ne!(listed[0]["id"], draft_id);
    let questions = listed[0]["questions"]
        .as_array()
        .expect("learner-safe questions should be listed");
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0]["id"], question_id);
    assert_eq!(questions[0]["assessment_id"], published_id);
    assert_eq!(
        questions[0]["text"],
        "Which Rust concept prevents aliasing bugs?"
    );
    assert_eq!(
        questions[0]["options"],
        json!(["Ownership", "Prototype chains"])
    );
    assert!(questions[0].get("correct_answer").is_none());

    let attempts_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/assessments/{}/attempts",
            course.id, published_id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let attempts_response = test::call_service(&app, attempts_req).await;
    assert_eq!(attempts_response.status(), StatusCode::OK);
    let attempts_json: Value = test::read_body_json(attempts_response).await;
    let attempts = attempts_json.as_array().expect("attempt list response");
    assert_eq!(attempts.len(), 2);
    assert_eq!(attempts[0]["id"], newer_attempt_id);
    assert_eq!(attempts[1]["id"], older_attempt_id);
    assert!(attempts
        .iter()
        .all(|attempt| attempt["user_id"] == learner.id()));
}
