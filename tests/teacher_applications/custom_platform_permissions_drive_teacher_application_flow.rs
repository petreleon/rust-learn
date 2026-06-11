#[actix_web::test]
async fn custom_platform_permissions_drive_teacher_application_flow() {
    let mut conn = setup_conn().await;
    let applicant = create_user_helper(&mut conn, "custom_teacher_apply_user").await;
    let reviewer = create_user_helper(&mut conn, "custom_teacher_reviewer").await;
    let review_only_user = create_user_helper(&mut conn, "custom_teacher_review_only").await;

    let submit_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_SUBMITTER"),
        &[Permissions::SUBMIT_TEACHER_APPLICATION],
    )
    .await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_APPROVER"),
        &[
            Permissions::REVIEW_TEACHER_APPLICATIONS,
            Permissions::APPROVE_TEACHER_APPLICATION,
        ],
    )
    .await;
    let review_only_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("TEACHER_APPLICATION_REVIEW_ONLY"),
        &[Permissions::REVIEW_TEACHER_APPLICATIONS],
    )
    .await;
    assign_platform_role_id(&mut conn, applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;
    assign_platform_role_id(&mut conn, review_only_user.id(), review_only_role_id).await;

    let application = submit_application(&mut conn, applicant.id(), platform_application_request())
        .await
        .expect("submit permission should create teacher application");

    let visible = list_applications(
        &mut conn,
        reviewer.id(),
        ListTeacherApplicationsRequest {
            applicant_user_id: Some(applicant.id()),
            ..Default::default()
        },
    )
    .await
    .expect("review permission should list teacher applications");
    assert!(visible.iter().any(|item| item.id == application.id));

    let denied_approval = decide_application(
        &mut conn,
        review_only_user.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("review-only user attempted approval".to_string()),
        },
    )
    .await
    .expect_err("review-only permission must not approve teacher applications");
    assert!(matches!(
        denied_approval,
        TeacherApplicationError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_TEACHER_APPLICATION.to_string()
    ));

    let approved = decide_application(
        &mut conn,
        reviewer.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("custom permission reviewer approved".to_string()),
        },
    )
    .await
    .expect("approve permission should approve teacher application");
    assert_eq!(approved.status, TEACHER_APPLICATION_STATUS_APPROVED);
    assert_eq!(approved.reviewer_id, Some(reviewer.id()));
}
