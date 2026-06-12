use actix_web::{http::StatusCode, test, web, App};
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{assessment_attempts, assessment_questions, assessments, courses};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::user::User;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
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

async fn create_assessment(conn: &mut AsyncPgConnection, course_id: i32) -> i32 {
    diesel::insert_into(assessments::table)
        .values((
            assessments::course_id.eq(course_id),
            assessments::title.eq("Submit quiz"),
            assessments::passing_score.eq(60),
            assessments::max_attempts.eq(1),
            assessments::published.eq(true),
        ))
        .returning(assessments::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment")
}

async fn create_question(
    conn: &mut AsyncPgConnection,
    assessment_id: i32,
    order: i32,
    correct_answer: &str,
    points: i32,
) -> i32 {
    diesel::insert_into(assessment_questions::table)
        .values((
            assessment_questions::assessment_id.eq(assessment_id),
            assessment_questions::text.eq(format!("Question {}", order)),
            assessment_questions::question_type.eq("multiple_choice"),
            assessment_questions::correct_answer.eq(Some(correct_answer.to_string())),
            assessment_questions::points.eq(points),
            assessment_questions::order.eq(order),
        ))
        .returning(assessment_questions::id)
        .get_result(conn)
        .await
        .expect("failed to create assessment question")
}

fn submit_body(first_question_id: i32, second_question_id: i32) -> Value {
    let mut answers = serde_json::Map::new();
    answers.insert(first_question_id.to_string(), json!(" yes "));
    answers.insert(second_question_id.to_string(), json!("wrong"));
    json!({ "answers": Value::Object(answers) })
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn assessment_submit_attempt_scores_persists_and_enforces_max_attempts() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_test_user(&mut conn, "assessment_submit").await;
    let course = create_course(&mut conn, &unique_string("AssessmentSubmitCourse")).await;
    let assessment_id = create_assessment(&mut conn, course.id).await;
    let yes_question_id = create_question(&mut conn, assessment_id, 1, "yes", 2).await;
    let no_question_id = create_question(&mut conn, assessment_id, 2, "no", 3).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;
    let uri = format!(
        "/courses/{}/assessments/{}/submit",
        course.id, assessment_id
    );
    let token = token_for(learner.id());

    let first_req = test::TestRequest::post()
        .uri(&uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(submit_body(yes_question_id, no_question_id))
        .to_request();
    let first_response = test::call_service(&app, first_req).await;
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_json: Value = test::read_body_json(first_response).await;
    assert_eq!(first_json["score"], 2);
    assert_eq!(first_json["total_points"], 5);
    assert_eq!(first_json["percentage"], 40);
    assert_eq!(first_json["passed"], false);
    assert_eq!(first_json["attempt"]["assessment_id"], assessment_id);
    assert_eq!(first_json["attempt"]["user_id"], learner.id());
    assert_eq!(first_json["attempt"]["score"], 2);
    assert_eq!(first_json["attempt"]["passed"], false);
    assert!(first_json["attempt"]["completed_at"].is_string());

    let second_req = test::TestRequest::post()
        .uri(&uri)
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .set_json(submit_body(yes_question_id, no_question_id))
        .to_request();
    let second_response = test::call_service(&app, second_req).await;
    assert_eq!(second_response.status(), StatusCode::FORBIDDEN);

    let mut conn = setup_conn(&pool).await;
    let attempt_count: i64 = assessment_attempts::table
        .filter(assessment_attempts::assessment_id.eq(assessment_id))
        .filter(assessment_attempts::user_id.eq(learner.id()))
        .count()
        .get_result(&mut conn)
        .await
        .expect("attempt count should load");
    assert_eq!(attempt_count, 1);
}
