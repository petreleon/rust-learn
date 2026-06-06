use actix_web::{http::StatusCode, test, web, App};
use bcrypt::verify;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rust_learn::db::schema::{
    authentications, email_verification_tokens, user_role_platform, users,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::role::PlatformRole;
use rust_learn::models::user::User;
use rust_learn::utils::jwt_utils::decode_jwt;

fn unique_email(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    format!("{}+{}-{}@example.com", prefix, std::process::id(), ts)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

fn auth_test_app(
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
        .service(web::scope("/api").service(rust_learn::api::authentication::auth_scope()))
}

#[actix_web::test]
async fn register_creates_unverified_user_auth_role_and_verification_token() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let email = unique_email("auth-register");
    let password = "ValidPass123!";

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": password,
            "name": "Auth Register",
            "date_of_birth": "2001-02-03"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body.as_ref(), b"Registration successful");

    let mut conn = setup_conn(&pool).await;
    let user = User::find_by_email(&email, &mut conn)
        .await
        .expect("registered user should exist");
    assert_eq!(user.name, "Auth Register");
    assert_eq!(
        user.date_of_birth,
        Some(NaiveDate::from_ymd_opt(2001, 2, 3).unwrap())
    );
    assert!(!user.kyc_verified);
    assert!(!user.email_verified);

    let password_hash: Option<String> = authentications::table
        .filter(authentications::user_id.eq(user.id()))
        .filter(authentications::type_authentication.eq("password"))
        .select(authentications::info_auth)
        .first(&mut conn)
        .await
        .expect("password authentication should exist");
    let password_hash = password_hash.expect("password hash should be stored");
    assert!(verify(password, &password_hash).expect("password hash should be valid bcrypt"));

    let student_role_id = PlatformRole::find_by_name("STUDENT", &mut conn)
        .await
        .expect("STUDENT role should be seeded");
    let role_assignments: i64 = user_role_platform::table
        .filter(user_role_platform::user_id.eq(user.id()))
        .filter(user_role_platform::platform_role_id.eq(student_role_id))
        .count()
        .get_result(&mut conn)
        .await
        .expect("role assignment query should succeed");
    assert_eq!(role_assignments, 1);

    let active_tokens: i64 = email_verification_tokens::table
        .filter(email_verification_tokens::user_id.eq(user.id()))
        .filter(email_verification_tokens::used_at.is_null())
        .count()
        .get_result(&mut conn)
        .await
        .expect("verification token query should succeed");
    assert_eq!(active_tokens, 1);
}

#[actix_web::test]
async fn login_requires_email_verification_and_returns_jwt_after_verification() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let email = unique_email("auth-login");
    let password = "ValidPass123!";

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": password,
            "name": "Auth Login",
            "date_of_birth": "2002-04-05"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::FORBIDDEN);
    let body = test::read_body(login_resp).await;
    assert_eq!(body.as_ref(), b"Email verification required");

    let mut conn = setup_conn(&pool).await;
    let user = User::find_by_email(&email, &mut conn)
        .await
        .expect("registered user should exist");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(&mut conn)
        .await
        .expect("test should mark user as email verified");

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);

    let jwt: String = test::read_body_json(login_resp).await;
    let claims = decode_jwt(&jwt).expect("login should return a valid JWT");
    assert_eq!(claims.claims.user_id, user.id());
}

#[actix_web::test]
async fn register_rejects_weak_password_and_login_rejects_bad_credentials() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let weak_email = unique_email("auth-weak-password");
    let weak_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": weak_email,
            "password": "short",
            "name": "Weak Password",
            "date_of_birth": "2003-06-07"
        }))
        .to_request();
    let weak_resp = test::call_service(&app, weak_req).await;
    assert_eq!(weak_resp.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(weak_resp).await;
    assert_eq!(
        body.as_ref(),
        b"Password must be at least 12 characters long"
    );

    let email = unique_email("auth-bad-login");
    let password = "ValidPass123!";
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": password,
            "name": "Bad Login",
            "date_of_birth": "2004-08-09"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    let bad_login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": "WrongPass123!"
        }))
        .to_request();
    let bad_login_resp = test::call_service(&app, bad_login_req).await;
    assert_eq!(bad_login_resp.status(), StatusCode::UNAUTHORIZED);
    let body = test::read_body(bad_login_resp).await;
    assert_eq!(body.as_ref(), b"Invalid credentials");
}

#[actix_web::test]
async fn register_rejects_duplicate_email_without_panicking() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let email = unique_email("auth-duplicate");
    let password = "ValidPass123!";
    let payload = serde_json::json!({
        "email": email,
        "password": password,
        "name": "Duplicate Register",
        "date_of_birth": "2005-10-11"
    });

    let first_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();
    let first_resp = test::call_service(&app, first_req).await;
    assert_eq!(first_resp.status(), StatusCode::OK);

    let duplicate_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&payload)
        .to_request();
    let duplicate_resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(duplicate_resp.status(), StatusCode::CONFLICT);
    let body = test::read_body(duplicate_resp).await;
    assert_eq!(body.as_ref(), b"Email already registered");

    let mut conn = setup_conn(&pool).await;
    let user_count: i64 = users::table
        .filter(users::email.eq(email))
        .count()
        .get_result(&mut conn)
        .await
        .expect("duplicate registration should leave one user");
    assert_eq!(user_count, 1);
}
