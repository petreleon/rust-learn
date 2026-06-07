// src/services/jwt_middleware.rs
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures::future::{ok, ready, Either, Ready};
use jsonwebtoken::errors::ErrorKind;
use std::task::{Context, Poll};

use crate::models::user_jwt::UserJWT;
use crate::utils::jwt_utils::decode_jwt;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let auth_str = match auth_header.to_str() {
                Ok(auth_str) => auth_str,
                Err(_) => {
                    return Either::Right(ready(Err(ErrorUnauthorized(
                        "Invalid Authorization header format",
                    ))))
                }
            };

            let Some(token) = auth_str.strip_prefix("Bearer ") else {
                return Either::Right(ready(Err(ErrorUnauthorized(
                    "Invalid Authorization header format",
                ))));
            };

            let token_data = match decode_jwt(token) {
                Ok(token_data) => token_data,
                Err(error) => {
                    let message = match error.kind() {
                        ErrorKind::ExpiredSignature => "Token expired",
                        _ => "Invalid token",
                    };
                    return Either::Right(ready(Err(ErrorUnauthorized(message))));
                }
            };

            let user_jwt: UserJWT = token_data.claims;
            let exp = user_jwt.exp;
            let now = chrono::Utc::now().timestamp() as usize;
            if exp < now {
                return Either::Right(ready(Err(ErrorUnauthorized("Token expired"))));
            }
            req.extensions_mut().insert(user_jwt);
        }

        Either::Left(self.service.call(req))
    }
}

#[cfg(test)]
mod tests {
    use super::JwtMiddleware;
    use crate::models::user_jwt::UserJWT;
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

        encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
            .expect("test JWT should encode")
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
}
