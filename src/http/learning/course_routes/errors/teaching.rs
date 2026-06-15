use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn teacher_course_dashboard_read_error(
    error: TeacherCourseDashboardError,
) -> ApiError {
    match error {
        TeacherCourseDashboardError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to view this teaching course")
        }
        TeacherCourseDashboardError::NotFound => super::course_not_found(),
        TeacherCourseDashboardError::Connection(message) => {
            super::db_connection_failed("teacher_course_dashboard_connection_failed", message)
        }
        TeacherCourseDashboardError::Database(message) => super::logged_internal(
            "teacher_course_dashboard_failed",
            "",
            "Failed to load teaching courses",
            message,
        ),
    }
}
