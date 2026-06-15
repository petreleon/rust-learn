use crate::{
    create_custom_platform_role::*, decision_support::*, platform_review_support::*,
    submit_support::*, support::*,
};
#[actix_web::test]
async fn platform_teacher_application_review_contract_returns_context_and_filters() {
    let mut conn = setup_conn().await;
    let org = create_organization(&mut conn, &unique_string("platform_teacher_review_org")).await;
    let course = create_course(&mut conn, &unique_string("Platform Async Teaching Lab")).await;
    let search_marker = unique_string("platform_review_marker");
    let submitted_applicant = create_user_helper(&mut conn, "platform_review_submitted").await;
    let approved_applicant = create_user_helper(&mut conn, "platform_review_approved").await;
    let reviewer = create_user_helper(&mut conn, "platform_review_reviewer").await;
    let outsider = create_user_helper(&mut conn, "platform_review_outsider").await;

    let submit_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEW_SUBMITTER"),
        &[Permissions::SUBMIT_TEACHER_APPLICATION],
    )
    .await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEWER"),
        &[
            Permissions::REVIEW_TEACHER_APPLICATIONS,
            Permissions::APPROVE_TEACHER_APPLICATION,
            Permissions::REJECT_TEACHER_APPLICATION,
        ],
    )
    .await;
    assign_platform_role_id(&mut conn, submitted_applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, approved_applicant.id(), submit_role_id).await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;

    let submitted = submit_application(
        &mut conn,
        submitted_applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "course".to_string(),
            requested_organization_id: None,
            requested_course_id: Some(course.id),
            experience_summary: format!(
                "Async Rust mentor with production examples. {search_marker}"
            ),
            organization_sponsor_id: Some(org.id),
            portfolio_links: Some(vec!["https://example.test/async-mentor".to_string()]),
            idempotency_key: None,
        },
    )
    .await
    .expect("submitted fixture should be created");

    let approved = submit_application(
        &mut conn,
        approved_applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "organization".to_string(),
            requested_organization_id: Some(org.id),
            requested_course_id: None,
            experience_summary: "Organization teaching lead.".to_string(),
            organization_sponsor_id: Some(org.id),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("approved fixture should be created");
    decide_application(
        &mut conn,
        reviewer.id(),
        approved.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("experienced organization teacher".to_string()),
        },
    )
    .await
    .expect("reviewer should approve fixture");

    let response = list_platform_applications(
        &mut conn,
        reviewer.id(),
        PlatformTeacherApplicationsRequest {
            status: Some("submitted".to_string()),
            search: Some(search_marker.clone()),
            limit: Some(10),
            offset: Some(0),
        },
    )
    .await
    .expect("reviewer should load platform review queue");

    assert_eq!(response.total, 1);
    assert_eq!(response.limit, 10);
    assert_eq!(response.offset, 0);
    assert_eq!(response.status.as_deref(), Some("submitted"));
    assert_eq!(response.search.as_deref(), Some(search_marker.as_str()));
    assert!(response.summary.total >= 2);
    assert!(response.summary.submitted >= 1);
    assert!(response.summary.approved >= 1);
    assert!(response.operator_permissions.can_view_applications);
    assert!(response.operator_permissions.can_approve_applications);
    assert!(response.operator_permissions.can_reject_applications);
    assert!(response.operator_permissions.can_request_changes);

    let application = response
        .applications
        .as_slice()
        .first()
        .expect("filtered application should be returned");
    assert_eq!(application.id, submitted.id);
    assert_eq!(application.applicant.id, submitted_applicant.id());
    assert_eq!(application.applicant.name, submitted_applicant.name);
    assert_eq!(application.requested_scope, "course");
    assert_eq!(
        application
            .requested_course
            .as_ref()
            .map(|course| course.title.as_str()),
        Some(course.title.as_str())
    );
    assert_eq!(
        application
            .sponsor_organization
            .as_ref()
            .map(|organization| organization.name.as_str()),
        Some(org.name.as_str())
    );
    assert_eq!(
        application.portfolio_links,
        vec!["https://example.test/async-mentor".to_string()]
    );
    assert_eq!(application.audit.event_count, 1);
    assert_eq!(
        application
            .audit
            .latest_event_type
            .map(|event_type| event_type.as_str()),
        Some("submitted")
    );

    let denied = list_platform_applications(
        &mut conn,
        outsider.id(),
        PlatformTeacherApplicationsRequest::default(),
    )
    .await
    .expect_err("users without review permission cannot load platform queue");
    assert!(matches!(
        denied,
        rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewError::PermissionDenied(permission)
            if permission == Permissions::REVIEW_TEACHER_APPLICATIONS.to_string()
    ));

    let app = test::init_service(
        App::new()
            .app_data(teacher_application_platform_review_data())
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::teacher_applications::configure_routes),
    )
    .await;
    let http_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!(
                "/teacher-applications/review?status=submitted&search={search_marker}"
            ))
            .insert_header((
                "Authorization",
                format!("Bearer {}", token_for(reviewer.id())),
            ))
            .to_request(),
    )
    .await;
    assert_eq!(http_response.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(http_response).await;
    assert!(body["applications"]
        .as_array()
        .expect("applications should be an array")
        .iter()
        .any(|application| application["id"].as_i64() == Some(submitted.id)));
}
