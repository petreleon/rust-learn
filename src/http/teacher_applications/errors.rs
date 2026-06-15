use actix_web::http::StatusCode;

use crate::application::teacher_applications::decide_application::TeacherApplicationDecisionError;
use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfError;
use crate::application::teacher_applications::list_application_audit::TeacherApplicationAuditError;
use crate::application::teacher_applications::list_applications::TeacherApplicationListError;
use crate::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewError;
use crate::application::teacher_applications::nominate_application::TeacherApplicationNominationError;
use crate::application::teacher_applications::submit_application::TeacherApplicationSubmitError;
use crate::http::errors::ApiError;

const REQUIRED_PERMISSION: &str = "User does not have the required permission";
const PROCESSING_FAILED: &str = "Failed to process teacher application";

pub(super) fn submit_application_error(error: TeacherApplicationSubmitError) -> ApiError {
    match error {
        TeacherApplicationSubmitError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationSubmitError::InvalidInput(message) => invalid_input(message),
        TeacherApplicationSubmitError::InvalidTransition(message) => invalid_transition(message),
        TeacherApplicationSubmitError::Connection(message)
        | TeacherApplicationSubmitError::Database(message) => {
            internal("teacher_application_submit_api_failed", message)
        }
    }
}

pub(super) fn self_application_error(error: TeacherApplicationSelfError) -> ApiError {
    match error {
        TeacherApplicationSelfError::Connection(message)
        | TeacherApplicationSelfError::Database(message) => {
            internal("teacher_application_self_api_failed", message)
        }
    }
}

pub(super) fn list_applications_error(error: TeacherApplicationListError) -> ApiError {
    match error {
        TeacherApplicationListError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationListError::InvalidInput(message) => invalid_input(message),
        TeacherApplicationListError::Connection(message)
        | TeacherApplicationListError::Database(message) => {
            internal("teacher_application_list_api_failed", message)
        }
    }
}

pub(super) fn nomination_error(error: TeacherApplicationNominationError) -> ApiError {
    match error {
        TeacherApplicationNominationError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationNominationError::InvalidInput(message) => invalid_input(message),
        TeacherApplicationNominationError::InvalidTransition(message) => {
            invalid_transition(message)
        }
        TeacherApplicationNominationError::NotFound => teacher_application_not_found(),
        TeacherApplicationNominationError::Connection(message)
        | TeacherApplicationNominationError::Database(message) => {
            internal("teacher_application_nomination_api_failed", message)
        }
    }
}

pub(super) fn decision_error(error: TeacherApplicationDecisionError) -> ApiError {
    match error {
        TeacherApplicationDecisionError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationDecisionError::InvalidInput(message) => invalid_input(message),
        TeacherApplicationDecisionError::InvalidTransition(message) => invalid_transition(message),
        TeacherApplicationDecisionError::NotFound => teacher_application_not_found(),
        TeacherApplicationDecisionError::Connection(message)
        | TeacherApplicationDecisionError::Database(message) => {
            internal("teacher_application_decision_api_failed", message)
        }
    }
}

pub(super) fn audit_error(error: TeacherApplicationAuditError) -> ApiError {
    match error {
        TeacherApplicationAuditError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationAuditError::Connection(message)
        | TeacherApplicationAuditError::Database(message) => {
            internal("teacher_application_audit_api_failed", message)
        }
    }
}

pub(super) fn platform_review_error(error: TeacherApplicationPlatformReviewError) -> ApiError {
    match error {
        TeacherApplicationPlatformReviewError::PermissionDenied(_) => permission_denied(),
        TeacherApplicationPlatformReviewError::InvalidInput(message) => invalid_input(message),
        TeacherApplicationPlatformReviewError::Connection(message)
        | TeacherApplicationPlatformReviewError::Database(message) => {
            internal("teacher_application_platform_review_api_failed", message)
        }
    }
}

fn permission_denied() -> ApiError {
    ApiError::new(
        StatusCode::FORBIDDEN,
        "permission_denied",
        REQUIRED_PERMISSION,
    )
}

fn invalid_input(message: String) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
}

fn invalid_transition(message: String) -> ApiError {
    ApiError::new(
        StatusCode::CONFLICT,
        "invalid_teacher_application_transition",
        message,
    )
}

fn teacher_application_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "teacher_application_not_found",
        "Teacher application not found",
    )
}

fn internal(event: &'static str, message: String) -> ApiError {
    log::error!("event={} error={}", event, message);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "teacher_application_processing_failed",
        PROCESSING_FAILED,
    )
}

#[cfg(test)]
mod tests {
    use super::{decision_error, list_applications_error};
    use crate::application::teacher_applications::decide_application::TeacherApplicationDecisionError;
    use crate::application::teacher_applications::list_applications::TeacherApplicationListError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn permission_denied_uses_api_error_envelope() {
        let response = list_applications_error(TeacherApplicationListError::PermissionDenied(
            "review".to_string(),
        ))
        .error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::FORBIDDEN);
        assert_eq!(body.value["error"]["code"], "permission_denied");
        assert_eq!(body.value["error"]["message"], super::REQUIRED_PERMISSION);
    }

    #[actix_web::test]
    async fn not_found_uses_teacher_application_envelope() {
        let response = decision_error(TeacherApplicationDecisionError::NotFound).error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::NOT_FOUND);
        assert_eq!(body.value["error"]["code"], "teacher_application_not_found");
        assert_eq!(
            body.value["error"]["message"],
            "Teacher application not found"
        );
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
