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

    let list_app = test::init_service(
        App::new()
            .app_data(teacher_application_list_data())
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::teacher_applications::configure_routes),
    )
    .await;
    let list_response = test::call_service(
        &list_app,
        test::TestRequest::get()
            .uri(&format!(
                "/teacher-applications?status={}",
                TEACHER_APPLICATION_STATUS_SUBMITTED
            ))
            .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
            .to_request(),
    )
    .await;
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body: serde_json::Value = test::read_body_json(list_response).await;
    assert!(list_body
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["id"].as_i64() == Some(application.id)));

    let decision_app = test::init_service(
        App::new()
            .app_data(teacher_application_decision_data())
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::teacher_applications::configure_routes),
    )
    .await;
    let decision_response = test::call_service(
        &decision_app,
        test::TestRequest::put()
            .uri(&format!("/teacher-applications/{}/decision", application.id))
            .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
            .set_json(TeacherApplicationDecisionRequest {
                status: "approved".to_string(),
                decision_reason: Some("candidate approved by central administration".to_string()),
            })
            .to_request(),
    )
    .await;
    assert_eq!(decision_response.status(), StatusCode::OK);
    let decision_body: serde_json::Value = test::read_body_json(decision_response).await;
    assert_eq!(
        decision_body["status"].as_str(),
        Some(TEACHER_APPLICATION_STATUS_APPROVED)
    );
    assert_eq!(
        decision_body["reviewer_id"].as_i64(),
        Some(i64::from(admin.id()))
    );

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

    let app = test::init_service(
        App::new()
            .app_data(teacher_application_audit_data())
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::teacher_applications::configure_routes),
    )
    .await;
    let response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/teacher-applications/{}/audit", application.id))
            .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
            .to_request(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 2);
    assert_eq!(
        body[1]["to_status"].as_str(),
        Some(TEACHER_APPLICATION_STATUS_APPROVED)
    );
}
