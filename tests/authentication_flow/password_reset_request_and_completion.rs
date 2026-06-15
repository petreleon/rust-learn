use crate::support::*;

#[actix_web::test]
async fn password_reset_request_is_private_and_completion_is_one_time() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let email = unique_email("auth-reset");
    let old_password = "ValidPass123!";
    let new_password = "BetterPass123!";

    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": old_password,
            "name": "Reset User",
            "date_of_birth": "2000-01-02"
        }))
        .to_request();
    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    let user = find_user_by_email(&mut conn, &email)
        .await
        .expect("registered user should exist");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(true))
        .execute(&mut conn)
        .await
        .expect("test should mark user verified");

    for requested_email in [&email, "missing-reset@example.com"] {
        let req = test::TestRequest::post()
            .uri("/api/auth/forgot-password")
            .set_json(serde_json::json!({ "email": requested_email }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(
            body.as_ref(),
            b"If an account matches that email, a password reset link has been sent."
        );
    }

    let active_tokens: i64 = password_reset_tokens::table
        .filter(password_reset_tokens::user_id.eq(user.id()))
        .filter(password_reset_tokens::used_at.is_null())
        .count()
        .get_result(&mut conn)
        .await
        .expect("reset token query should succeed");
    assert_eq!(active_tokens, 1);

    let reset_token = unique_token("reset-token");
    create_password_reset_token(&mut conn, user.id(), identity_token_hash(&reset_token))
        .await
        .expect("test should create reset token");

    let weak_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": &reset_token, "password": "short" }))
        .to_request();
    let weak_resp = test::call_service(&app, weak_req).await;
    assert_eq!(weak_resp.status(), StatusCode::BAD_REQUEST);

    let reset_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": &reset_token, "password": new_password }))
        .to_request();
    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status(), StatusCode::OK);

    let old_login = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({ "email": email, "password": old_password }))
        .to_request();
    assert_eq!(
        test::call_service(&app, old_login).await.status(),
        StatusCode::UNAUTHORIZED
    );

    let new_login = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({ "email": email, "password": new_password }))
        .to_request();
    assert_eq!(
        test::call_service(&app, new_login).await.status(),
        StatusCode::OK
    );

    let replay_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": &reset_token, "password": "AnotherPass123!" }))
        .to_request();
    let replay_resp = test::call_service(&app, replay_req).await;
    assert_eq!(replay_resp.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(replay_resp).await;
    assert_eq!(body.as_ref(), b"Invalid password reset token");
}

#[actix_web::test]
async fn reset_password_rejects_missing_invalid_and_expired_tokens() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;

    let missing_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": "", "password": "BetterPass123!" }))
        .to_request();
    let missing_resp = test::call_service(&app, missing_req).await;
    assert_eq!(missing_resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        test::read_body(missing_resp).await.as_ref(),
        b"Password reset token is required"
    );

    let invalid_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": "not-a-token", "password": "BetterPass123!" }))
        .to_request();
    let invalid_resp = test::call_service(&app, invalid_req).await;
    assert_eq!(invalid_resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        test::read_body(invalid_resp).await.as_ref(),
        b"Invalid password reset token"
    );

    let email = unique_email("auth-reset-expired");
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": "ValidPass123!",
            "name": "Expired Reset",
            "date_of_birth": "2000-01-02"
        }))
        .to_request();
    assert_eq!(
        test::call_service(&app, register_req).await.status(),
        StatusCode::OK
    );

    let mut conn = setup_conn(&pool).await;
    let user = find_user_by_email(&mut conn, &email)
        .await
        .expect("registered user should exist");
    let expired_token = unique_token("expired-reset-token");
    create_password_reset_token(&mut conn, user.id(), identity_token_hash(&expired_token))
        .await
        .expect("test should create reset token");
    diesel::update(
        password_reset_tokens::table
            .filter(password_reset_tokens::token_hash.eq(identity_token_hash(&expired_token))),
    )
    .set(password_reset_tokens::expires_at.eq(chrono::Utc::now().naive_utc()))
    .execute(&mut conn)
    .await
    .expect("test should expire reset token");

    let expired_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({ "token": &expired_token, "password": "BetterPass123!" }))
        .to_request();
    let expired_resp = test::call_service(&app, expired_req).await;
    assert_eq!(expired_resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        test::read_body(expired_resp).await.as_ref(),
        b"Password reset token expired"
    );
}
