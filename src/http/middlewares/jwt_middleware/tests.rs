use super::JwtMiddleware;
use crate::application::identity::auth_token::{
    AuthTokenVerificationError, AuthTokenVerifier, AuthTokenVerifierService,
};
use crate::domain::identity::UserJWT;
use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App, HttpMessage, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use std::sync::Arc;

struct FakeTokenVerifier;

impl AuthTokenVerifier for FakeTokenVerifier {
    fn verify_token(&self, token: &str) -> Result<UserJWT, AuthTokenVerificationError> {
        match token {
            "valid" => Ok(UserJWT::new(42, Utc::now() + Duration::minutes(5))),
            "expired" => Err(AuthTokenVerificationError::Expired),
            _ => Err(AuthTokenVerificationError::Invalid),
        }
    }
}

fn verifier_data() -> web::Data<AuthTokenVerifierService> {
    web::Data::new(Arc::new(FakeTokenVerifier))
}

fn response_status<B>(
    result: Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
) -> StatusCode {
    match result {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    }
}

#[actix_web::test]
async fn passes_requests_without_authorization_header() {
    let app = test::init_service(
        App::new()
            .wrap(JwtMiddleware)
            .route("/probe", web::get().to(HttpResponse::Ok)),
    )
    .await;

    let req = test::TestRequest::get().uri("/probe").to_request();

    assert_eq!(response_status(app.call(req).await), StatusCode::OK);
}

#[actix_web::test]
async fn inserts_valid_bearer_claims() {
    let app = test::init_service(
        App::new()
            .app_data(verifier_data())
            .wrap(JwtMiddleware)
            .route(
                "/probe",
                web::get().to(|req: HttpRequest| async move {
                    let user_id = req
                        .extensions()
                        .get::<UserJWT>()
                        .map(|claims| claims.user_id)
                        .unwrap_or_default();

                    HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id }))
                }),
            ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", "Bearer valid"))
        .to_request();

    let response = test::call_service(&app, req).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["user_id"], 42);
}

#[actix_web::test]
async fn rejects_expired_bearer_tokens() {
    let app = test::init_service(
        App::new()
            .app_data(verifier_data())
            .wrap(JwtMiddleware)
            .route("/probe", web::get().to(HttpResponse::Ok)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", "Bearer expired"))
        .to_request();

    assert_eq!(
        response_status(app.call(req).await),
        StatusCode::UNAUTHORIZED
    );
}

#[actix_web::test]
async fn rejects_malformed_authorization_header() {
    let app = test::init_service(
        App::new()
            .wrap(JwtMiddleware)
            .route("/probe", web::get().to(HttpResponse::Ok)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", "Basic credentials"))
        .to_request();

    assert_eq!(
        response_status(app.call(req).await),
        StatusCode::UNAUTHORIZED
    );
}

#[actix_web::test]
async fn rejects_bearer_tokens_without_registered_verifier() {
    let app = test::init_service(
        App::new()
            .wrap(JwtMiddleware)
            .route("/probe", web::get().to(HttpResponse::Ok)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", "Bearer valid"))
        .to_request();

    assert_eq!(
        response_status(app.call(req).await),
        StatusCode::UNAUTHORIZED
    );
}
