use crate::application::learning::assign_course_role::CourseRoleAssignmentError;
use crate::application::learning::course_enrollment::CourseEnrollmentError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn course_enrollment_error(
    error: CourseEnrollmentError,
) -> ApiError {
    match error {
        CourseEnrollmentError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to manage enrollment")
        }
        CourseEnrollmentError::InvalidStatus(message) => super::invalid_input(message),
        CourseEnrollmentError::NotFound => ApiError::new(
            actix_web::http::StatusCode::NOT_FOUND,
            "course_enrollment_not_found",
            "Course enrollment not found",
        ),
        CourseEnrollmentError::Connection(message) => {
            super::db_connection_failed("course_enrollment_connection_failed", message)
        }
        CourseEnrollmentError::Database(message) => super::logged_internal(
            "course_enrollment_failed",
            "",
            "Failed to manage course enrollment",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_role_assignment_error(
    error: CourseRoleAssignmentError,
) -> ApiError {
    match error {
        CourseRoleAssignmentError::HierarchyViolation => super::permission_denied(
            "Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.",
        ),
        CourseRoleAssignmentError::NotFound => super::invalid_input("Role or User not found"),
        CourseRoleAssignmentError::Connection(message) => {
            super::db_connection_failed("course_role_assign_connection_failed", message)
        }
        CourseRoleAssignmentError::Database(message) => super::logged_internal(
            "course_role_assign_failed",
            "",
            "Failed to assign role",
            message,
        ),
    }
}
