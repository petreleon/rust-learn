#[actix_web::test]
async fn teacher_application_submission_is_idempotent_by_key() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_idempotent").await;
    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let idempotency_key = unique_string("teacher_application_submit");
    let mut request = platform_application_request();
    request.idempotency_key = Some(idempotency_key.clone());

    let application = submit_application(&mut conn, applicant.id(), request.clone())
        .await
        .expect("initial teacher application should be created");
    let duplicate = submit_application(&mut conn, applicant.id(), request)
        .await
        .expect("same idempotency key and payload should return existing application");
    assert_eq!(duplicate.id, application.id);
    assert_eq!(
        duplicate.idempotency_key.as_deref(),
        Some(idempotency_key.as_str())
    );

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(
        audit.len(),
        1,
        "idempotent replay should not add another submitted audit event"
    );

    let mut conflicting_request = platform_application_request();
    conflicting_request.idempotency_key = Some(idempotency_key);
    conflicting_request.experience_summary =
        "Different application payload for same retry key.".to_string();
    let denied = submit_application(&mut conn, applicant.id(), conflicting_request)
        .await
        .expect_err("idempotency key reuse for a different application should be rejected");
    assert!(matches!(
        denied,
        TeacherApplicationError::InvalidInput(message)
            if message.contains("idempotency key is already used")
    ));
}

#[actix_web::test]
async fn applicant_can_read_latest_application_snapshot_without_review_permission() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_snapshot").await;
    let stranger = create_user_helper(&mut conn, "teacher_apply_snapshot_empty").await;
    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let snapshot = get_my_application(&mut conn, applicant.id())
        .await
        .expect("applicant should read own application snapshot");
    assert_eq!(
        snapshot.application.as_ref().map(|item| item.id),
        Some(application.id)
    );
    assert_eq!(snapshot.audit_events.len(), 1);
    assert_eq!(snapshot.audit_events[0].application_id, application.id);
    assert_eq!(snapshot.audit_events[0].to_status, application.status);

    let app = test::init_service(
        App::new()
            .app_data(teacher_application_self_data())
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::teacher_applications::configure_routes),
    )
    .await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/teacher-applications/me")
            .insert_header(("Authorization", format!("Bearer {}", token_for(applicant.id()))))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["application"]["id"].as_i64(), Some(application.id));
    assert_eq!(body["audit_events"].as_array().unwrap().len(), 1);

    let empty_snapshot = get_my_application(&mut conn, stranger.id())
        .await
        .expect("authenticated user without application should get an empty snapshot");
    assert!(empty_snapshot.application.is_none());
    assert!(empty_snapshot.audit_events.is_empty());
}
