use std::sync::Arc;

use actix_web::web;
use serde::Serialize;

use crate::application::identity::jwks::{JsonWebKeyOutput, JwksOutput, JwksUseCase};
use crate::http::identity::authentication::errors::jwks_error;
use crate::http::identity::authentication::text_error::AuthTextError;

#[derive(Serialize)]
pub(crate) struct JwksResponse {
    keys: Vec<JsonWebKeyResponse>,
}

#[derive(Serialize)]
struct JsonWebKeyResponse {
    kty: String,
    #[serde(rename = "use")]
    public_key_use: String,
    kid: String,
    alg: String,
    n: String,
    e: String,
}

impl From<JwksOutput> for JwksResponse {
    fn from(output: JwksOutput) -> Self {
        Self {
            keys: output.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<JsonWebKeyOutput> for JsonWebKeyResponse {
    fn from(key: JsonWebKeyOutput) -> Self {
        Self {
            kty: key.kty,
            public_key_use: key.public_key_use,
            kid: key.kid,
            alg: key.alg,
            n: key.n,
            e: key.e,
        }
    }
}

pub(crate) async fn jwks(
    use_case: web::Data<Arc<dyn JwksUseCase>>,
) -> Result<web::Json<JwksResponse>, AuthTextError> {
    use_case
        .jwks()
        .await
        .map(JwksResponse::from)
        .map(web::Json)
        .map_err(jwks_error)
}

#[cfg(test)]
mod tests {
    use super::jwks;
    use crate::application::identity::jwks::{
        JsonWebKeyOutput, JwksError, JwksOutput, JwksUseCase,
    };
    use actix_web::{http::StatusCode, test, web, App};
    use futures::future::{ready, BoxFuture, FutureExt};
    use std::sync::Arc;

    struct FakeJwksUseCase;

    impl JwksUseCase for FakeJwksUseCase {
        fn jwks(&self) -> BoxFuture<'_, Result<JwksOutput, JwksError>> {
            ready(Ok(JwksOutput {
                keys: vec![JsonWebKeyOutput {
                    kty: "RSA".to_string(),
                    public_key_use: "sig".to_string(),
                    kid: "test-key".to_string(),
                    alg: "RS256".to_string(),
                    n: "modulus".to_string(),
                    e: "AQAB".to_string(),
                }],
            }))
            .boxed()
        }
    }

    #[actix_web::test]
    async fn returns_jwks_from_application_use_case() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(FakeJwksUseCase) as Arc<dyn JwksUseCase>
                ))
                .route("/.well-known/jwks.json", web::get().to(jwks)),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/.well-known/jwks.json")
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(body["keys"][0]["kid"], "test-key");
        assert_eq!(body["keys"][0]["use"], "sig");
    }
}
