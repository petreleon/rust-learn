use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{courses, notifications};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::course_join_request::COURSE_JOIN_STATUS_APPROVED;
use rust_learn::models::role::{CourseRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use rust_learn::utils::notifications::NotificationsState;
use serde_json::Value;

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
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
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role should exist");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

fn course_enrollment_test_app(
    pool: DbPool,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(NotificationsState::new(pool)))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(rust_learn::api::courses::course_scope())
}

async fn notification_count(conn: &mut AsyncPgConnection, user_id: i32, title: &str) -> i64 {
    notifications::table
        .filter(notifications::user_id.eq(Some(user_id)))
        .filter(notifications::title.eq(title))
        .count()
        .get_result(conn)
        .await
        .expect("notification count should load")
}

#[actix_web::test]
async fn course_join_approval_api_sends_enrollment_notification() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let student = create_test_user(&mut conn, "api_join_student").await;
    let teacher = create_test_user(&mut conn, "api_join_teacher").await;
    let course = create_course(&mut conn, &unique_string("ApiJoinCourse")).await;
    assign_platform_role(&mut conn, student.id(), "USER").await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let app = test::init_service(course_enrollment_test_app(pool.clone())).await;

    let request_join = test::TestRequest::post()
        .uri(&format!("/courses/{}/join-requests", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let response = test::call_service(&app, request_join).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    let request_id = body["id"].as_i64().expect("join request id");

    let approve_join = test::TestRequest::put()
        .uri(&format!(
            "/courses/{}/join-requests/{}/decision",
            course.id, request_id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(serde_json::json!({
            "status": COURSE_JOIN_STATUS_APPROVED,
            "decision_reason": "welcome"
        }))
        .to_request();
    let response = test::call_service(&app, approve_join).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["status"].as_str(), Some(COURSE_JOIN_STATUS_APPROVED));

    let mut conn = setup_conn(&pool).await;
    assert_eq!(
        notification_count(&mut conn, student.id(), "course:enrolled").await,
        1
    );
}

#[actix_web::test]
async fn course_role_assignment_api_does_not_send_enrollment_notification() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let teacher = create_test_user(&mut conn, "api_role_teacher").await;
    let target = create_test_user(&mut conn, "api_role_target").await;
    let course = create_course(&mut conn, &unique_string("ApiRoleCourse")).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let app = test::init_service(course_enrollment_test_app(pool.clone())).await;

    let assign_role = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/users/{}/roles",
            course.id,
            target.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();
    let response = test::call_service(&app, assign_role).await;
    assert_eq!(response.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(
        notification_count(&mut conn, target.id(), "role:assigned").await,
        1
    );
    assert_eq!(
        notification_count(&mut conn, target.id(), "course:enrolled").await,
        0
    );
}
