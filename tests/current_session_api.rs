use actix_web::{http::StatusCode, test, web, App};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{courses, organizations, users};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::course::{Course, NewCourse};
use rust_learn::models::delegated_permission::NewDelegatedPermission;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::role::{CourseRole, OrganizationRole, PlatformRole};
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_organization::UserRoleOrganization;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::delegated_permission_repository::create_delegated_permission;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
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

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
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

async fn assign_organization_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    role_name: &str,
) {
    let role_id = OrganizationRole::find_by_name(role_name, conn)
        .await
        .expect("organization role should exist");
    UserRoleOrganization::assign(conn, user_id, organization_id, role_id)
        .await
        .expect("failed to assign organization role");
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

fn current_session_test_app(
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
        .app_data(web::Data::new(pool))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(
            web::scope("/api").service(
                web::resource("/me")
                    .route(web::get().to(rust_learn::api::session::get_current_session)),
            ),
        )
}

fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}

#[actix_web::test]
async fn current_session_returns_profile_scopes_and_delegations() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let user = create_test_user(&mut conn, "current_session_user").await;
    let grantor = create_test_user(&mut conn, "current_session_grantor").await;
    let organization = create_organization(&mut conn, &unique_string("CurrentSessionOrg")).await;
    let course = create_course(&mut conn, &unique_string("CurrentSessionCourse")).await;

    assign_platform_role(&mut conn, user.id(), "ADMIN").await;
    assign_organization_role(&mut conn, user.id(), organization.id, "ADMIN").await;
    assign_course_role(&mut conn, user.id(), course.id, "TEACHER").await;

    create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: grantor.id(),
            grantee_user_id: user.id(),
            permission: "EXECUTE_REWARD_PAYOUT".to_string(),
            scope_type: "platform".to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("session API test".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("failed to create delegated permission");
    drop(conn);

    let app = test::init_service(current_session_test_app(pool.clone())).await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me")
            .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;

    assert_eq!(body["user"]["id"].as_i64(), Some(user.id() as i64));
    assert_eq!(body["user"]["email_verified"].as_bool(), Some(true));
    assert!(array_contains(&body["platform"]["roles"], "ADMIN"));
    assert!(array_contains(
        &body["platform"]["direct_permissions"],
        "VIEW_REPORT"
    ));
    assert!(array_contains(
        &body["platform"]["delegated_permissions"],
        "EXECUTE_REWARD_PAYOUT"
    ));
    assert!(array_contains(
        &body["platform"]["effective_permissions"],
        "EXECUTE_REWARD_PAYOUT"
    ));

    let organizations = body["organizations"]
        .as_array()
        .expect("organizations array");
    let session_org = organizations
        .iter()
        .find(|item| item["id"].as_i64() == Some(organization.id as i64))
        .expect("session should include organization scope");
    assert_eq!(
        session_org["name"].as_str(),
        Some(organization.name.as_str())
    );
    assert!(array_contains(&session_org["roles"], "ADMIN"));
    assert!(array_contains(
        &session_org["direct_permissions"],
        "GENERATE_REPORT"
    ));

    let courses = body["courses"].as_array().expect("courses array");
    let session_course = courses
        .iter()
        .find(|item| item["id"].as_i64() == Some(course.id as i64))
        .expect("session should include course scope");
    assert_eq!(
        session_course["title"].as_str(),
        Some(course.title.as_str())
    );
    assert!(array_contains(&session_course["roles"], "TEACHER"));
    assert!(array_contains(
        &session_course["direct_permissions"],
        "CREATE_CONTENT"
    ));

    let delegations = body["delegated_permissions"]
        .as_array()
        .expect("delegated permissions array");
    assert!(delegations.iter().any(|delegation| {
        delegation["permission"].as_str() == Some("EXECUTE_REWARD_PAYOUT")
            && delegation["scope_type"].as_str() == Some("platform")
    }));
}

#[actix_web::test]
async fn current_session_requires_authorization() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(current_session_test_app(pool.clone())).await;

    let response =
        test::call_service(&app, test::TestRequest::get().uri("/api/me").to_request()).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["error"]["code"].as_str(), Some("unauthorized"));
}

#[actix_web::test]
async fn current_session_rejects_unverified_email() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let user = create_test_user(&mut conn, "current_session_unverified").await;

    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(false))
        .execute(&mut conn)
        .await
        .expect("failed to mark user unverified");
    drop(conn);

    let app = test::init_service(current_session_test_app(pool.clone())).await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me")
            .insert_header(("Authorization", format!("Bearer {}", token_for(user.id()))))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["error"]["code"].as_str(), Some("unverified_email"));
}

#[actix_web::test]
async fn current_session_reports_missing_user() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(current_session_test_app(pool.clone())).await;

    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/me")
            .insert_header(("Authorization", format!("Bearer {}", token_for(i32::MAX))))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["error"]["code"].as_str(), Some("missing_user"));
}
