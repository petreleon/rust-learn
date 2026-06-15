use actix_web::{error::JsonPayloadError, http::StatusCode, web};

use crate::http::errors::ApiError;

pub const JSON_PAYLOAD_LIMIT_BYTES: usize = 2 * 1024 * 1024;

pub fn configure_json(cfg: &mut web::ServiceConfig) {
    cfg.app_data(json_config());
}

fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(JSON_PAYLOAD_LIMIT_BYTES)
        .error_handler(|error, _request| json_payload_error(error).into())
}

fn json_payload_error(error: JsonPayloadError) -> ApiError {
    match error {
        JsonPayloadError::ContentType => ApiError::new(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "unsupported_json_content_type",
            "Request body must use Content-Type application/json.",
        ),
        JsonPayloadError::Overflow { .. } | JsonPayloadError::OverflowKnownLength { .. } => {
            ApiError::new(
                StatusCode::PAYLOAD_TOO_LARGE,
                "json_payload_too_large",
                format!("JSON request body exceeds the {JSON_PAYLOAD_LIMIT_BYTES} byte limit."),
            )
        }
        JsonPayloadError::Deserialize(_) => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_json",
            "Request body must contain valid JSON.",
        ),
        JsonPayloadError::Payload(_) => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_json_payload",
            "Request JSON payload could not be read.",
        ),
        JsonPayloadError::Serialize(_) => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_json_payload",
            "Request JSON payload could not be processed.",
        ),
        _ => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_json",
            "Request body must contain valid JSON.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{configure_json, JSON_PAYLOAD_LIMIT_BYTES};
    use actix_web::{http::header, http::StatusCode, test, web, App};
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    struct EchoBody {
        name: String,
    }

    async fn echo(body: web::Json<EchoBody>) -> web::Json<Value> {
        web::Json(serde_json::json!({ "name": body.name }))
    }

    #[actix_web::test]
    async fn malformed_json_uses_api_error_envelope() {
        let app = test::init_service(
            App::new()
                .configure(configure_json)
                .route("/echo", web::post().to(echo)),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/echo")
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .set_payload("{")
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = test::read_body_json(response).await;
        assert_eq!(body["error"]["code"], "invalid_json");
        assert_eq!(body["error"]["status"], 400);
    }

    #[actix_web::test]
    async fn non_json_content_type_uses_api_error_envelope() {
        let app = test::init_service(
            App::new()
                .configure(configure_json)
                .route("/echo", web::post().to(echo)),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/echo")
                .insert_header((header::CONTENT_TYPE, "text/plain"))
                .set_payload(r#"{"name":"Ada"}"#)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let body: Value = test::read_body_json(response).await;
        assert_eq!(body["error"]["code"], "unsupported_json_content_type");
        assert_eq!(body["error"]["status"], 415);
    }

    #[actix_web::test]
    async fn oversized_json_uses_api_error_envelope() {
        let oversized_name = "x".repeat(JSON_PAYLOAD_LIMIT_BYTES);
        let payload = format!(r#"{{"name":"{oversized_name}"}}"#);
        let app = test::init_service(
            App::new()
                .configure(configure_json)
                .route("/echo", web::post().to(echo)),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/echo")
                .insert_header((header::CONTENT_TYPE, "application/json"))
                .insert_header((header::CONTENT_LENGTH, payload.len().to_string()))
                .set_payload(payload.into_bytes())
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
        let body: Value = test::read_body_json(response).await;
        assert_eq!(body["error"]["code"], "json_payload_too_large");
        assert_eq!(body["error"]["status"], 413);
    }
}
