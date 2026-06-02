use actix_web::{http::StatusCode, test, web, App};
use rust_learn::db::establish_connection;
use rust_learn::utils::s3_utils::S3State;
use serde_json::Value;

#[actix_web::test]
async fn health_returns_ok_without_dependencies() {
    let app = test::init_service(App::new().service(rust_learn::api::health::health_scope())).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "ok");
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
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(s3))
            .service(rust_learn::api::health::health_scope()),
    )
    .await;

    let req = test::TestRequest::get().uri("/ready").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
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
