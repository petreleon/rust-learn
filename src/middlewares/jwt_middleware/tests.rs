use super::JwtMiddleware;
use crate::domain::identity::UserJWT;
use actix_service::Service;
use actix_web::{http::StatusCode, test, web, App, HttpMessage, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

fn encode_test_token(user_id: i32, exp: chrono::DateTime<Utc>) -> String {
    dotenvy::dotenv().ok();
    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set for tests");
    let claims = UserJWT::new(user_id, exp);
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())
        .expect("PRIVATE_KEY must be an RSA private key");

    encode(&Header::new(Algorithm::RS256), &claims, &encoding_key).expect("test JWT should encode")
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
    let token = encode_test_token(42, Utc::now() + Duration::minutes(5));
    let app = test::init_service(App::new().wrap(JwtMiddleware).route(
        "/probe",
        web::get().to(|req: HttpRequest| async move {
            let user_id = req
                .extensions()
                .get::<UserJWT>()
                .map(|claims| claims.user_id)
                .unwrap_or_default();

            HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id }))
        }),
    ))
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let response = test::call_service(&app, req).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["user_id"], 42);
}

#[actix_web::test]
async fn rejects_expired_bearer_tokens() {
    let token = encode_test_token(42, Utc::now() - Duration::minutes(5));
    let app = test::init_service(
        App::new()
            .wrap(JwtMiddleware)
            .route("/probe", web::get().to(HttpResponse::Ok)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/probe")
        .insert_header(("Authorization", format!("Bearer {token}")))
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
