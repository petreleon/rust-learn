use actix_web::http::StatusCode;

use crate::application::kyc::KycError;
use crate::http::errors::ApiError;

const REQUIRED_PERMISSION: &str = "User does not have the required permission";

pub(super) fn kyc_error(error: KycError) -> ApiError {
    match error {
        KycError::PermissionDenied(_) => ApiError::new(
            StatusCode::FORBIDDEN,
            "permission_denied",
            REQUIRED_PERMISSION,
        ),
        KycError::InvalidInput(message) => {
            ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
        }
        KycError::InvalidTransition(message) => {
            ApiError::new(StatusCode::CONFLICT, "invalid_kyc_transition", message)
        }
        KycError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "kyc_submission_not_found",
            "KYC submission not found",
        ),
        KycError::Connection(message) | KycError::Database(message) => {
            log::error!("event=kyc_api_failed reason=database error={}", message);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "kyc_request_failed",
                "Failed to process KYC request",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::kyc_error;
    use crate::application::kyc::KycError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn permission_denied_uses_api_error_envelope() {
        let response =
            kyc_error(KycError::PermissionDenied("review_kyc".to_string())).error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::FORBIDDEN);
        assert_eq!(body.value["error"]["code"], "permission_denied");
        assert_eq!(body.value["error"]["message"], super::REQUIRED_PERMISSION);
        assert_eq!(body.value["error"]["status"], 403);
    }

    #[actix_web::test]
    async fn invalid_transition_uses_conflict_envelope() {
        let response = kyc_error(KycError::InvalidTransition(
            "cannot approve twice".to_string(),
        ))
        .error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::CONFLICT);
        assert_eq!(body.value["error"]["code"], "invalid_kyc_transition");
        assert_eq!(body.value["error"]["message"], "cannot approve twice");
        assert_eq!(body.value["error"]["status"], 409);
    }

    #[actix_web::test]
    async fn not_found_uses_kyc_specific_envelope() {
        let response = kyc_error(KycError::NotFound).error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::NOT_FOUND);
        assert_eq!(body.value["error"]["code"], "kyc_submission_not_found");
        assert_eq!(body.value["error"]["message"], "KYC submission not found");
        assert_eq!(body.value["error"]["status"], 404);
    }

    struct ParsedErrorBody {
        status: StatusCode,
        value: Value,
    }

    async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
        let status = response.status();
        let body = to_bytes(response.into_body()).await.unwrap();
        let value = serde_json::from_slice(&body).unwrap();
        ParsedErrorBody { status, value }
    }
}
