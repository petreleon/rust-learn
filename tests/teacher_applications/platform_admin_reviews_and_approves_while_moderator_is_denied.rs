#[actix_web::test]
async fn platform_admin_reviews_and_approves_while_moderator_is_denied() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "teacher_review_applicant").await;
    let admin = create_user_helper(&mut conn, "teacher_review_admin").await;
    let moderator = create_user_helper(&mut conn, "teacher_review_moderator").await;

    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    assign_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("application should be created");

    let denied = decide_application(
        &mut conn,
        moderator.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("moderators do not review teacher applications".to_string()),
        },
    )
    .await
    .expect_err("moderator should not approve teacher applications by role name");
    assert!(matches!(
        denied,
        TeacherApplicationError::PermissionDenied(_)
    ));

    let pending = list_applications(
        &mut conn,
        admin.id(),
        ListTeacherApplicationsRequest {
            status: Some(TEACHER_APPLICATION_STATUS_SUBMITTED.to_string()),
            ..Default::default()
        },
    )
    .await
    .expect("admin should list teacher applications");
    assert!(pending.iter().any(|item| item.id == application.id));

    let approved = decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("candidate approved by central administration".to_string()),
        },
    )
    .await
    .expect("admin should approve teacher application");
    assert_eq!(approved.status, TEACHER_APPLICATION_STATUS_APPROVED);
    assert_eq!(approved.reviewer_id, Some(admin.id()));

    let applicant_has_teacher_bundle_permission = user_permission_platform_request(
        &mut conn,
        applicant.id(),
        &Permissions::GENERATE_REPORT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_has_teacher_bundle_permission,
        "approved platform-scope applicant should receive the platform teaching bundle"
    );

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].to_status, TEACHER_APPLICATION_STATUS_SUBMITTED);
    assert_eq!(audit[1].to_status, TEACHER_APPLICATION_STATUS_APPROVED);
}
