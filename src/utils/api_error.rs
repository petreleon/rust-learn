use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub error: ApiErrorBody,
}

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    pub status: u16,
}

pub fn api_error_response(status: u16, code: &str, message: &str) -> HttpResponse {
    HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    )
    .json(ApiError {
        error: ApiErrorBody {
            code: code.to_string(),
            message: message.to_string(),
            status,
        },
    })
}

pub fn api_not_found(message: &str) -> HttpResponse {
    api_error_response(404, "not_found", message)
}

pub fn api_bad_request(message: &str) -> HttpResponse {
    api_error_response(400, "bad_request", message)
}

pub fn api_internal_error(message: &str) -> HttpResponse {
    api_error_response(500, "internal_error", message)
}
