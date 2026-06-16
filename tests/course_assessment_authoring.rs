use std::sync::Arc;

use actix_web::{http::StatusCode, test, web, App};
use chrono::{NaiveDate, Utc};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::application::learning::manage_assessments::AssessmentAuthoringUseCase;
use rust_learn::infra::postgres::access_control::{course_role_records, role_catalog_store};
use rust_learn::infra::postgres::identity::bootstrap_accounts::create_verified_password_user;
use rust_learn::infra::postgres::learning::assessment_authoring_use_case::PostgresAssessmentAuthoringUseCase;
use rust_learn::infra::postgres::models::course::{Course, NewCourse};
use rust_learn::infra::postgres::models::user::User;
use rust_learn::infra::postgres::schema::courses;
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

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("course role assignment failed");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn assessment_authoring_data(pool: &DbPool) -> web::Data<Arc<dyn AssessmentAuthoringUseCase>> {
    web::Data::new(Arc::new(PostgresAssessmentAuthoringUseCase::new(
        pool.clone(),
    )))
}

#[actix_web::test]
async fn teacher_authors_and_publishes_assessment_with_answer_key() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "assessment_author").await;
    let course = create_course(&mut conn, &unique_string("AssessmentAuthorCourse")).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(assessment_authoring_data(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;
    let token = token_for(teacher.id());
    let authoring_uri = format!("/courses/{}/assessments/authoring", course.id);

    let invalid_publish = test::TestRequest::post()
        .uri(&authoring_uri)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "Final quiz",
            "description": null,
            "passing_score": 80,
            "max_attempts": 2,
            "published": true,
            "questions": []
        }))
        .to_request();
    let invalid_response = test::call_service(&app, invalid_publish).await;
    assert_eq!(invalid_response.status(), StatusCode::BAD_REQUEST);

    let create_req = test::TestRequest::post()
        .uri(&authoring_uri)
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(json!({
            "title": "Draft quiz",
            "description": "Ownership checks",
            "passing_score": 70,
            "max_attempts": 3,
            "published": false,
            "questions": []
        }))
        .to_request();
    let create_response = test::call_service(&app, create_req).await;
    assert_eq!(create_response.status(), StatusCode::OK);
    let created: Value = test::read_body_json(create_response).await;
    let assessment_id = created["id"].as_i64().expect("created assessment id");
    assert_eq!(created["published"], false);
    assert_eq!(created["questions"].as_array().unwrap().len(), 0);

    let update_uri = format!(
        "/courses/{}/assessments/{}/authoring",
        course.id, assessment_id
    );
    let update_req = test::TestRequest::put()
        .uri(&update_uri)
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(json!({
            "title": "Published quiz",
            "description": "Ownership checks",
            "passing_score": 70,
            "max_attempts": 3,
            "published": true,
            "questions": [{
                "text": "Which concept prevents aliasing bugs?",
                "question_type": "multiple_choice",
                "options": ["Ownership", "Prototype chains"],
                "correct_answer": "Ownership",
                "points": 2,
                "order": 0
            }]
        }))
        .to_request();
    let update_response = test::call_service(&app, update_req).await;
    assert_eq!(update_response.status(), StatusCode::OK);
    let updated: Value = test::read_body_json(update_response).await;
    assert_eq!(updated["published"], true);
    assert_eq!(updated["questions"][0]["correct_answer"], "Ownership");

    let list_req = test::TestRequest::get()
        .uri(&authoring_uri)
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .to_request();
    let list_response = test::call_service(&app, list_req).await;
    assert_eq!(list_response.status(), StatusCode::OK);
    let listed: Value = test::read_body_json(list_response).await;
    assert_eq!(listed[0]["questions"][0]["correct_answer"], "Ownership");
}
