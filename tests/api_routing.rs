use actix_web::{
    http::{Method, StatusCode},
    test, web, App, HttpResponse,
};

#[actix_web::test]
async fn api_scope_and_following_routes_are_reachable() {
    let app = test::init_service(
        App::new()
            .route(
                "/hey",
                web::get().to(|| async { HttpResponse::Ok().body("hey") }),
            )
            .configure(rust_learn::api::health::configure_health_routes)
            .service(rust_learn::api::api_scope())
            .route(
                "/after-api",
                web::get().to(|| async { HttpResponse::Ok().body("after") }),
            ),
    )
    .await;

    let hey = test::call_service(&app, test::TestRequest::get().uri("/hey").to_request()).await;
    assert_eq!(hey.status(), StatusCode::OK);

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

    for (method, route) in [
        (Method::GET, "/api/courses"),
        (Method::GET, "/api/teacher-applications"),
        (Method::GET, "/api/teacher-applications/me"),
        (Method::POST, "/api/teacher-applications"),
        (Method::PUT, "/api/teacher-applications/34/decision"),
        (Method::GET, "/api/courses/teaching"),
        (Method::GET, "/api/courses/teaching/12"),
        (Method::GET, "/api/reward-candidates/me/history"),
        (Method::GET, "/api/courses/12/reward-candidates"),
        (
            Method::PUT,
            "/api/courses/12/reward-candidates/34/teacher-decision",
        ),
        (Method::PUT, "/api/reward-candidates/34/amount-decision"),
        (
            Method::POST,
            "/api/organizations/56/courses/12/reward-candidates",
        ),
        (Method::GET, "/api/reports/platform/summary"),
        (Method::GET, "/api/reports/platform/summary.csv"),
        (Method::GET, "/api/reports/platform/reward-dashboard"),
        (Method::GET, "/api/reports/platform/reward-dashboard.csv"),
        (Method::GET, "/api/reports/platform/fraud-dashboard"),
        (Method::GET, "/api/reports/platform/fraud-dashboard.csv"),
        (
            Method::GET,
            "/api/reports/platform/teacher-applications.csv",
        ),
        (Method::GET, "/api/reports/platform/reward-approvals.csv"),
        (Method::GET, "/api/reports/platform/token-payouts.csv"),
        (Method::GET, "/api/reports/platform/wallet-credits.csv"),
        (
            Method::GET,
            "/api/reports/platform/delegated-permissions.csv",
        ),
        (Method::GET, "/api/reports/organizations/56/summary"),
        (Method::GET, "/api/reports/organizations/56/summary.csv"),
        (
            Method::GET,
            "/api/reports/organizations/56/reward-dashboard",
        ),
        (
            Method::GET,
            "/api/reports/organizations/56/reward-dashboard.csv",
        ),
        (Method::GET, "/api/reward-fraud-blocks"),
        (Method::POST, "/api/reward-fraud-blocks"),
        (Method::PUT, "/api/reward-fraud-blocks/78/revoke"),
        (Method::GET, "/api/reward-fraud-blocks/78/audit"),
        (Method::GET, "/api/delegated-permissions"),
        (Method::POST, "/api/delegated-permissions"),
        (Method::PUT, "/api/delegated-permissions/90/revoke"),
    ] {
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
