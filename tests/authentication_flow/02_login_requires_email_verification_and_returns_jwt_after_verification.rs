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
