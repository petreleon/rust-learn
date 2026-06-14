use actix_web::{http::StatusCode, test, web, App};
use rust_learn::application::operations::readiness_check::ReadinessUseCase;
use rust_learn::bootstrap::readiness::RuntimeReadinessUseCase;
use rust_learn::db::establish_connection;
use rust_learn::infra::object_storage::S3State;
use serde_json::Value;
use std::sync::Arc;

#[actix_web::test]
async fn health_returns_ok_without_dependencies() {
    let app =
        test::init_service(App::new().service(rust_learn::http::operations::health_scope())).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "ok");
}

#[actix_web::test]
async fn readiness_reports_not_ready_without_configured_use_case() {
    let app =
        test::init_service(App::new().service(rust_learn::http::operations::health_scope())).await;

    let req = test::TestRequest::get().uri("/ready").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "not_ready");
}

#[actix_web::test]
async fn readiness_checks_database_s3_and_ethereum() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let s3 = S3State::new_from_env()
        .await
        .expect("S3 state should initialize from env");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(readiness_use_case_data(pool, s3))
            .service(rust_learn::http::operations::health_scope()),
    )
    .await;

    let req = test::TestRequest::get().uri("/ready").to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body: Value = test::read_body_json(resp).await;

    assert_eq!(status, StatusCode::OK, "readiness response: {body}");
    assert_eq!(body["status"], "ready");

    let checks = body["checks"]
        .as_array()
        .expect("checks should be an array");
    assert_eq!(checks.len(), 3);
    assert!(checks.iter().any(|check| check["name"] == "postgres"
        && check["status"] == "ok"
        && check["message"].is_null()));
    assert!(checks.iter().any(|check| check["name"] == "s3"
        && check["status"] == "ok"
        && check["message"].is_null()));
    assert!(checks.iter().any(|check| check["name"] == "ethereum"
        && check["status"] == "ok"
        && check["message"].is_null()));
}

fn readiness_use_case_data(
    pool: rust_learn::db::DbPool,
    s3: S3State,
) -> web::Data<Arc<dyn ReadinessUseCase>> {
    web::Data::new(Arc::new(RuntimeReadinessUseCase::new(pool, s3)))
}
