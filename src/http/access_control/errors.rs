use actix_web::http::StatusCode;

use crate::application::access_control::manage_delegated_permissions::DelegatedPermissionError;
use crate::application::access_control::role_catalog::RoleCatalogError;
use crate::http::errors::ApiError;

pub(super) fn role_catalog_error(scope: &'static str, error: RoleCatalogError) -> ApiError {
    match error {
        RoleCatalogError::Connection(_) => ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "db_connection_failed",
            "Failed to get DB connection",
        ),
        RoleCatalogError::Database(message) => {
            log::error!(
                "event=role_catalog_list_failed scope={} error={}",
                scope,
                message
            );
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "role_catalog_unavailable",
                "Error loading roles",
            )
        }
    }
}

pub(super) fn delegated_permission_error(error: DelegatedPermissionError) -> ApiError {
    match error {
        DelegatedPermissionError::PermissionDenied(_) => ApiError::new(
            StatusCode::FORBIDDEN,
            "permission_denied",
            "User does not have delegated-permission access",
        ),
        DelegatedPermissionError::InvalidInput(message) => {
            ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
        }
        DelegatedPermissionError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "delegated_permission_not_found",
            "Delegated permission not found",
        ),
        DelegatedPermissionError::Connection(message)
        | DelegatedPermissionError::Database(message) => {
            log::error!("event=delegated_permission_api_failed error={}", message);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "delegated_permission_unavailable",
                "Failed to process delegated permission",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{delegated_permission_error, role_catalog_error};
    use crate::application::access_control::manage_delegated_permissions::DelegatedPermissionError;
    use crate::application::access_control::role_catalog::RoleCatalogError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn role_catalog_connection_error_uses_api_error_envelope() {
        let response = role_catalog_error(
            "platform",
            RoleCatalogError::Connection("closed".to_string()),
        )
        .error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body.value["error"]["code"], "db_connection_failed");
        assert_eq!(
            body.value["error"]["message"],
            "Failed to get DB connection"
        );
        assert_eq!(body.value["error"]["status"], 500);
    }

    #[actix_web::test]
    async fn delegated_permission_invalid_input_uses_api_error_envelope() {
        let response =
            delegated_permission_error(DelegatedPermissionError::InvalidInput("bad scope".into()))
                .error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::BAD_REQUEST);
        assert_eq!(body.value["error"]["code"], "invalid_input");
        assert_eq!(body.value["error"]["message"], "bad scope");
        assert_eq!(body.value["error"]["status"], 400);
    }

    #[actix_web::test]
    async fn delegated_permission_not_found_uses_api_error_envelope() {
        let response =
            delegated_permission_error(DelegatedPermissionError::NotFound).error_response();

        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::NOT_FOUND);
        assert_eq!(
            body.value["error"]["code"],
            "delegated_permission_not_found"
        );
        assert_eq!(
            body.value["error"]["message"],
            "Delegated permission not found"
        );
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
