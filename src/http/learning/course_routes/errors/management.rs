use crate::application::learning::create_course::CourseCreationError;
use crate::application::learning::delete_course::CourseDeletionError;
use crate::application::learning::update_course::CourseUpdateError;
use crate::application::learning::update_course_lifecycle::CourseLifecycleError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn course_creation_error(
    error: CourseCreationError,
) -> ApiError {
    match error {
        CourseCreationError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to create course")
        }
        CourseCreationError::Connection(message) => {
            super::db_connection_failed("course_creation_connection_failed", message)
        }
        CourseCreationError::Database(message) => super::logged_internal(
            "course_creation_failed",
            "",
            "Failed to create course",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_update_error(
    error: CourseUpdateError,
) -> ApiError {
    match error {
        CourseUpdateError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to update course")
        }
        CourseUpdateError::NotFound => super::course_not_found(),
        CourseUpdateError::Connection(message) => {
            super::db_connection_failed("course_update_connection_failed", message)
        }
        CourseUpdateError::Database(message) => super::logged_internal(
            "course_update_failed",
            "",
            "Failed to update course",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_deletion_error(
    course_id: i32,
    error: CourseDeletionError,
) -> ApiError {
    match error {
        CourseDeletionError::Connection(message) => {
            super::db_connection_failed("course_delete_connection_failed", message)
        }
        CourseDeletionError::Database(message) => super::logged_internal(
            "course_delete_failed",
            &format!("course_id={course_id}"),
            "Failed to delete course",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn lifecycle_error(
    error: CourseLifecycleError,
) -> ApiError {
    match error {
        CourseLifecycleError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to update course status")
        }
        CourseLifecycleError::InvalidStatus(message) => super::invalid_input(message),
        CourseLifecycleError::NotFound => super::course_not_found(),
        CourseLifecycleError::Connection(message) => {
            super::db_connection_failed("course_lifecycle_connection_failed", message)
        }
        CourseLifecycleError::Database(message) => super::logged_internal(
            "course_lifecycle_update_failed",
            "",
            "Failed to update course status",
            message,
        ),
    }
}
