use crate::application::learning::manage_course_completion_terms::CourseCompletionTermsError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn course_completion_terms_error(
    error: CourseCompletionTermsError,
) -> ApiError {
    match error {
        CourseCompletionTermsError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to manage course terms")
        }
        CourseCompletionTermsError::InvalidInput(message) => super::invalid_input(message),
        CourseCompletionTermsError::InvalidStatus(message) => super::conflict(message),
        CourseCompletionTermsError::CourseNotFound => super::course_not_found(),
        CourseCompletionTermsError::TermsNotFound => ApiError::new(
            actix_web::http::StatusCode::NOT_FOUND,
            "course_completion_terms_not_found",
            "Course completion terms not found",
        ),
        CourseCompletionTermsError::Connection(message) => {
            super::db_connection_failed("course_completion_terms_connection_failed", message)
        }
        CourseCompletionTermsError::Database(message) => super::logged_internal(
            "course_completion_terms_failed",
            "",
            "Failed to manage course completion terms",
            message,
        ),
    }
}
