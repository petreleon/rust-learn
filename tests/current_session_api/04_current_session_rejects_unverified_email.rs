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
