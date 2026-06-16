use actix_web::http::StatusCode;

use crate::application::learning::assessment::AssessmentReadError;
use crate::application::learning::manage_assessments::AssessmentAuthoringError;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn course_assessments_error(
    course_id: i32,
    error: AssessmentReadError,
) -> ApiError {
    match error {
        AssessmentReadError::Connection(message) => super::db_unavailable(
            "assessments_list_connection_failed",
            &format!("course_id={course_id}"),
            message,
        ),
        AssessmentReadError::Database(message) => super::logged_internal(
            "assessments_list_failed",
            &format!("course_id={course_id}"),
            "Failed to list assessments",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn assessment_attempts_error(
    error: AssessmentReadError,
) -> ApiError {
    match error {
        AssessmentReadError::Connection(message) => {
            super::db_unavailable("assessment_attempts_connection_failed", "", message)
        }
        AssessmentReadError::Database(message) => super::logged_internal(
            "assessment_attempts_failed",
            "",
            "Failed to load attempts",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn assessment_submission_error(
    assessment_id: i32,
    error: AssessmentSubmissionError,
) -> ApiError {
    match error {
        AssessmentSubmissionError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "assessment_not_found",
            "Assessment not found",
        ),
        AssessmentSubmissionError::MaximumAttemptsReached => ApiError::new(
            StatusCode::FORBIDDEN,
            "maximum_attempts_reached",
            "Maximum attempts reached",
        ),
        AssessmentSubmissionError::Connection(message) => super::db_unavailable(
            "assessment_submit_connection_failed",
            &format!("assessment_id={assessment_id}"),
            message,
        ),
        AssessmentSubmissionError::LoadFailed(message) => super::logged_internal(
            "assessment_submit_lookup_failed",
            &format!("assessment_id={assessment_id}"),
            "Failed to load assessment",
            message,
        ),
        AssessmentSubmissionError::SaveFailed(message) => super::logged_internal(
            "assessment_submit_failed",
            &format!("assessment_id={assessment_id}"),
            "Failed to save attempt",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn assessment_authoring_error(
    course_id: i32,
    error: AssessmentAuthoringError,
) -> ApiError {
    match error {
        AssessmentAuthoringError::Connection(message) => super::db_unavailable(
            "assessment_authoring_connection_failed",
            &format!("course_id={course_id}"),
            message,
        ),
        AssessmentAuthoringError::Database(message) => super::logged_internal(
            "assessment_authoring_failed",
            &format!("course_id={course_id}"),
            "Failed to save assessment",
            message,
        ),
        AssessmentAuthoringError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "assessment_not_found",
            "Assessment not found",
        ),
        AssessmentAuthoringError::PermissionDenied(permission) => ApiError::new(
            StatusCode::FORBIDDEN,
            "permission_denied",
            format!("Missing required permission: {permission}"),
        ),
        AssessmentAuthoringError::Validation(message) => {
            ApiError::new(StatusCode::BAD_REQUEST, "invalid_assessment", message)
        }
    }
}
