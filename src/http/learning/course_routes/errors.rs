use actix_web::http::StatusCode;

use crate::http::errors::ApiError;

mod assessment;
mod catalog;
mod completion_terms;
mod enrollment;
mod management;
mod teaching;

pub(super) use assessment::{
    assessment_attempts_error, assessment_authoring_error, assessment_submission_error,
    course_assessments_error,
};
pub(super) use catalog::{
    course_discovery_error, course_organizations_error, course_read_error,
    learner_course_read_error, learner_progress_error,
};
pub(super) use completion_terms::course_completion_terms_error;
pub(super) use enrollment::{course_enrollment_error, course_role_assignment_error};
pub(super) use management::{
    course_creation_error, course_deletion_error, course_update_error, lifecycle_error,
};
pub(super) use teaching::teacher_course_dashboard_read_error;

pub(in crate::http::learning::course_routes) fn db_connection_failed(
    event: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} error={}", event, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(in crate::http::learning::course_routes) fn db_unavailable(
    event: &'static str,
    context: &str,
    error: String,
) -> ApiError {
    log::error!("event={} {} error={}", event, context, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "DB unavailable",
    )
}

pub(in crate::http::learning::course_routes) fn permission_denied(
    message: &'static str,
) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, "permission_denied", message)
}

pub(in crate::http::learning::course_routes) fn invalid_input(
    message: impl Into<String>,
) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_input", message)
}

pub(in crate::http::learning::course_routes) fn conflict(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::CONFLICT, "conflict", message)
}

pub(in crate::http::learning::course_routes) fn course_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "course_not_found",
        "Course not found",
    )
}

pub(in crate::http::learning::course_routes) fn logged_internal(
    event: &'static str,
    context: &str,
    message: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} {} error={}", event, context, error);
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "learning_request_failed",
        message,
    )
}

#[cfg(test)]
mod tests;
