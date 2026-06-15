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

#[actix_web::test]
async fn register_normalizes_email_and_login_accepts_case_variants() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let normalized_email = unique_email("auth-normalized");
    let submitted_email = format!("  {}  ", normalized_email.to_ascii_uppercase());
    let password = "ValidPass123!";

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": submitted_email,
            "password": password,
            "name": "Normalized Email",
            "date_of_birth": "2006-12-13"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    let user = find_user_by_email(&mut conn, &normalized_email)
        .await
        .expect("registered user should be stored under normalized email");
    assert_eq!(user.email, normalized_email);

    let duplicate_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": normalized_email.to_ascii_uppercase(),
            "password": password,
            "name": "Duplicate Normalized Email",
            "date_of_birth": "2007-01-14"
        }))
        .to_request();
    let duplicate_resp = test::call_service(&app, duplicate_req).await;
    assert_eq!(duplicate_resp.status(), StatusCode::CONFLICT);
    let body = test::read_body(duplicate_resp).await;
    assert_eq!(body.as_ref(), b"Email already registered");

    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(&mut conn)
        .await
        .expect("test should mark normalized user as email verified");

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": format!(" {} ", normalized_email.to_ascii_uppercase()),
            "password": password
        }))
        .to_request();
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);

    let jwt: String = test::read_body_json(login_resp).await;
    let claims = decode_jwt(&jwt).expect("login should return a valid JWT");
    assert_eq!(claims.claims.user_id, user.id());
}
