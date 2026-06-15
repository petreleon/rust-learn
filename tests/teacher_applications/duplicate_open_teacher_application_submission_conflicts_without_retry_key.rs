#[actix_web::test]
async fn duplicate_open_teacher_application_submission_conflicts_without_retry_key() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_duplicate").await;
    let admin = create_user_helper(&mut conn, "teacher_apply_duplicate_admin").await;
    assign_platform_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let duplicate = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect_err("open application should block duplicate submission");
    assert!(matches!(
        duplicate,
        TeacherApplicationError::InvalidTransition(message)
            if message.contains("already exists with status submitted")
    ));

    decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("candidate can reapply with stronger portfolio".to_string()),
        },
    )
    .await
    .expect("admin should reject teacher application");

    let resubmitted = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("rejected application should allow a new submission");
    assert_ne!(resubmitted.id, application.id);
    assert_eq!(resubmitted.status, TEACHER_APPLICATION_STATUS_SUBMITTED);
}

#[actix_web::test]
async fn user_with_submit_permission_can_apply_and_without_permission_cannot() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_apply_user").await;
    let no_permission_user = create_user_helper(&mut conn, "teacher_apply_denied").await;

    assign_platform_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");
    assert_eq!(application.applicant_user_id, applicant.id());
    assert_eq!(application.status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(
        application.portfolio_links,
        serde_json::json!(["https://example.com/portfolio"])
    );

    let denied = submit_application(
        &mut conn,
        no_permission_user.id(),
        platform_application_request(),
    )
    .await
    .expect_err("user without submit permission should be denied");
    assert!(matches!(
        denied,
        TeacherApplicationError::PermissionDenied(_)
    ));
}
