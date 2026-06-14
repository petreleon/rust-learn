use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    pub fn status_code_value(&self) -> u16 {
        self.status.as_u16()
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

#[derive(Serialize)]
struct ApiErrorEnvelope {
    error: ApiErrorBody,
}

#[derive(Serialize)]
struct ApiErrorBody {
    code: String,
    message: String,
    status: u16,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(ApiErrorEnvelope {
            error: ApiErrorBody {
                code: self.code.to_string(),
                message: self.message.clone(),
                status: self.status.as_u16(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ApiError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};

    #[actix_web::test]
    async fn renders_existing_error_envelope() {
        let error = ApiError::new(StatusCode::BAD_REQUEST, "bad_request", "Bad input");
        let response = error.error_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body()).await.unwrap();
        let text = std::str::from_utf8(&body).unwrap();

        assert!(text.contains("\"code\":\"bad_request\""));
        assert!(text.contains("\"message\":\"Bad input\""));
        assert!(text.contains("\"status\":400"));
    }
}
