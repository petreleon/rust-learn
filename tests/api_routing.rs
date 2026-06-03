use actix_web::{http::StatusCode, test, web, App, HttpResponse};

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

    let auth_hello = test::call_service(
        &app,
        test::TestRequest::get().uri("/api/auth/hello").to_request(),
    )
    .await;
    assert_eq!(auth_hello.status(), StatusCode::OK);

    let later_api_scope = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/api/reward-fraud-blocks")
            .to_request(),
    )
    .await;
    assert_ne!(later_api_scope.status(), StatusCode::NOT_FOUND);

    let after = test::call_service(
        &app,
        test::TestRequest::get().uri("/after-api").to_request(),
    )
    .await;
    assert_eq!(after.status(), StatusCode::OK);
}
