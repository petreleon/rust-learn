#[actix_web::test]
async fn verify_email_marks_user_verified_and_reports_already_verified_replay() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let app = test::init_service(auth_test_app(pool.clone())).await;
    let mut conn = setup_conn(&pool).await;

    let user = rust_learn::repositories::user_repository::create_user(
        &mut conn,
        "Verify Email",
        &unique_email("auth-verify"),
        Some(NaiveDate::from_ymd_opt(2004, 4, 4).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user");
    diesel::update(users::table.find(user.id()))
        .set(users::email_verified.eq(false))
        .execute(&mut conn)
        .await
        .expect("test should mark user unverified");

    let token = unique_token("verify-email-valid-token");
    EmailVerificationToken::create_for_user(&mut conn, user.id(), verification_token_hash(&token))
        .await
        .expect("failed to create verification token");
    drop(conn);

    let verify_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/auth/verify-email?token={token}"))
            .to_request(),
    )
    .await;
    assert_eq!(verify_resp.status(), StatusCode::OK);
    let body = test::read_body(verify_resp).await;
    assert_eq!(body.as_ref(), b"Email verified successfully");

    let mut conn = setup_conn(&pool).await;
    let verified = User::find_by_id(user.id(), &mut conn)
        .await
        .expect("user should still exist");
    assert!(verified.email_verified);
    drop(conn);

    let replay_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/auth/verify-email?token={token}"))
            .to_request(),
    )
    .await;
    assert_eq!(replay_resp.status(), StatusCode::OK);
    let body = test::read_body(replay_resp).await;
    assert_eq!(body.as_ref(), b"Email already verified");
}
