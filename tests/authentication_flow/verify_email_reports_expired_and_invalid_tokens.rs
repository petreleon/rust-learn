#[actix_web::test]
async fn verify_email_reports_expired_and_invalid_tokens() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;
    let mut conn = setup_conn(&pool).await;

    let user = rust_learn::repositories::user_repository::create_user(
        &mut conn,
        "Expired Verify Email",
        &unique_email("auth-expired-verify"),
        Some(NaiveDate::from_ymd_opt(2005, 5, 5).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(false))
        .execute(&mut conn)
        .await
        .expect("test should mark user unverified");

    let token = unique_token("verify-email-expired-token");
    let token_hash = identity_token_hash(&token);
    create_email_verification_token(&mut conn, user.id(), token_hash.clone())
        .await
        .expect("failed to create verification token");
    diesel::update(
        email_verification_tokens::table
            .filter(email_verification_tokens::token_hash.eq(token_hash)),
    )
    .set(
        email_verification_tokens::expires_at
            .eq(chrono::Utc::now().naive_utc() - chrono::Duration::minutes(1)),
    )
    .execute(&mut conn)
    .await
    .expect("failed to expire verification token");
    drop(conn);

    let expired_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/auth/verify-email?token={token}"))
            .to_request(),
    )
    .await;
    assert_eq!(expired_resp.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(expired_resp).await;
    assert_eq!(body.as_ref(), b"Verification token expired");

    let invalid_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/auth/verify-email?token=not-a-stored-token")
            .to_request(),
    )
    .await;
    assert_eq!(invalid_resp.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(invalid_resp).await;
    assert_eq!(body.as_ref(), b"Invalid verification token");
}

#[actix_web::test]
async fn register_rejects_blank_email() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "   ",
            "password": "ValidPass123!",
            "name": "Blank Email",
            "date_of_birth": "2003-07-08"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(resp).await;
    assert_eq!(body.as_ref(), b"Email is required");
}
