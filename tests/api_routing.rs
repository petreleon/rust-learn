use actix_web::http::StatusCode;
use actix_web::{test, web, App, HttpResponse};

mod api_routing_amount_decision_fake;
mod api_routing_course_creation_fake;
mod api_routing_course_deletion_fake;
mod api_routing_course_discovery_fake;
mod api_routing_course_lifecycle_fake;
mod api_routing_course_organization_fake;
mod api_routing_course_read_fake;
mod api_routing_course_update_fake;
mod api_routing_organization_courses_fake;
mod api_routing_organization_dashboard_fake;
mod api_routing_reporting_fake;
mod api_routing_route_fakes;
mod api_routing_route_targets;
mod api_routing_submission_fake;
mod api_routing_teacher_dashboard_fake;
mod api_routing_teacher_decision_fake;
mod api_routing_teacher_students_fake;

#[actix_web::test]
async fn bootstrap_routes_expose_index_and_health() {
    let app =
        test::init_service(App::new().configure(rust_learn::bootstrap::routes::configure_routes))
            .await;

    let index = test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(index.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(index).await;
    assert_eq!(body["service"].as_str(), Some("rust-learn-api"));
    assert_eq!(body["health"].as_str(), Some("/health"));
    assert_eq!(body["readiness"].as_str(), Some("/ready"));
    assert_eq!(body["api"].as_str(), Some("/api"));

    let health =
        test::call_service(&app, test::TestRequest::get().uri("/health").to_request()).await;
    assert_eq!(health.status(), StatusCode::OK);
}

#[actix_web::test]
async fn api_scope_and_following_routes_are_reachable() {
    let app = test::init_service(
        App::new()
            .app_data(api_routing_route_fakes::reward_fraud_block_data())
            .app_data(api_routing_route_fakes::reward_candidate_audit_data())
            .app_data(api_routing_amount_decision_fake::reward_amount_decision_data())
            .app_data(api_routing_submission_fake::reward_candidate_submission_data())
            .app_data(api_routing_course_creation_fake::course_creation_data())
            .app_data(api_routing_course_deletion_fake::course_deletion_data())
            .app_data(api_routing_course_discovery_fake::course_discovery_data())
            .app_data(api_routing_teacher_dashboard_fake::teacher_dashboard_data())
            .app_data(api_routing_teacher_dashboard_fake::teacher_workspace_data())
            .app_data(api_routing_teacher_dashboard_fake::teacher_enrollment_data())
            .app_data(api_routing_teacher_students_fake::teacher_students_data())
            .app_data(api_routing_course_lifecycle_fake::course_lifecycle_data())
            .app_data(api_routing_course_organization_fake::course_organizations_data())
            .app_data(api_routing_course_read_fake::course_read_data())
            .app_data(api_routing_course_update_fake::course_update_data())
            .app_data(api_routing_organization_dashboard_fake::organization_dashboard_data())
            .app_data(api_routing_organization_courses_fake::organization_course_list_data())
            .app_data(api_routing_organization_courses_fake::organization_member_audit_data())
            .app_data(api_routing_organization_courses_fake::organization_member_invite_data())
            .app_data(api_routing_organization_courses_fake::organization_member_list_data())
            .app_data(api_routing_organization_courses_fake::organization_member_removal_data())
            .app_data(api_routing_route_fakes::course_reward_candidates_data())
            .app_data(api_routing_route_fakes::platform_reward_candidates_data())
            .app_data(api_routing_teacher_decision_fake::teacher_reward_candidate_decision_data())
            .app_data(api_routing_route_fakes::student_reward_history_data())
            .app_data(api_routing_reporting_fake::organization_reward_dashboard_data())
            .app_data(api_routing_reporting_fake::organization_summary_data())
            .app_data(api_routing_reporting_fake::platform_csv_exports_data())
            .app_data(api_routing_reporting_fake::platform_fraud_dashboard_data())
            .app_data(api_routing_reporting_fake::platform_reward_dashboard_data())
            .app_data(api_routing_reporting_fake::platform_wallet_reconciliation_data())
            .configure(rust_learn::http::operations::configure_routes)
            .service(rust_learn::http::api_scope())
            .route(
                "/after-api",
                web::get().to(|| async { HttpResponse::Ok().body("after") }),
            ),
    )
    .await;

    let health =
        test::call_service(&app, test::TestRequest::get().uri("/health").to_request()).await;
    assert_eq!(health.status(), StatusCode::OK);

    let auth_user_id = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/auth/user_id")
            .to_request(),
    )
    .await;
    assert_ne!(auth_user_id.status(), StatusCode::NOT_FOUND);

    for (method, route) in api_routing_route_targets::route_smoke_targets() {
        match test::try_call_service(
            &app,
            test::TestRequest::default()
                .method(method)
                .uri(route)
                .to_request(),
        )
        .await
        {
            Ok(response) => assert_ne!(response.status(), StatusCode::NOT_FOUND, "{route}"),
            Err(error) => assert!(
                error.to_string().contains("database pool"),
                "{route} returned unexpected service error: {error}"
            ),
        }
    }

    let after = test::call_service(
        &app,
        test::TestRequest::get().uri("/after-api").to_request(),
    )
    .await;
    assert_eq!(after.status(), StatusCode::OK);
}
