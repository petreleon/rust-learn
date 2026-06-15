use crate::application::organizations::get_organization_dashboard::OrganizationDashboardError;
use crate::application::organizations::list_organization_courses::OrganizationCourseListError;
use crate::application::organizations::list_organization_teacher_applications::OrganizationTeacherApplicationListError;
use crate::http::errors::ApiError;

pub(in crate::http::organizations) fn organization_course_list_error(
    organization_id: i32,
    error: OrganizationCourseListError,
) -> ApiError {
    match error {
        OrganizationCourseListError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to view organization courses")
        }
        OrganizationCourseListError::NotFound => super::organization_not_found(),
        OrganizationCourseListError::Connection(error) => super::db_connection_failed(
            "organization_courses_connection_failed",
            format!("organization_id={organization_id}"),
            error,
        ),
        OrganizationCourseListError::Database(error) => super::logged_internal(
            "organization_courses_fetch_failed",
            &format!("organization_id={organization_id}"),
            "Failed to fetch organization courses",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_dashboard_error(
    organization_id: i32,
    error: OrganizationDashboardError,
) -> ApiError {
    match error {
        OrganizationDashboardError::PermissionDenied => {
            super::permission_denied("User does not have permission to view organization dashboard")
        }
        OrganizationDashboardError::NotFound => super::organization_not_found(),
        OrganizationDashboardError::Connection(error) => super::db_connection_failed(
            "organization_dashboard_connection_failed",
            format!("organization_id={organization_id}"),
            error,
        ),
        OrganizationDashboardError::Database(error)
        | OrganizationDashboardError::Reporting(error) => super::logged_internal(
            "organization_dashboard_fetch_failed",
            &format!("organization_id={organization_id}"),
            "Failed to fetch organization dashboard",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_teacher_application_list_error(
    organization_id: i32,
    error: OrganizationTeacherApplicationListError,
) -> ApiError {
    match error {
        OrganizationTeacherApplicationListError::PermissionDenied(_) => super::permission_denied(
            "User does not have permission to view organization teacher applications",
        ),
        OrganizationTeacherApplicationListError::InvalidInput(message) => {
            super::invalid_input(message)
        }
        OrganizationTeacherApplicationListError::NotFound => super::organization_not_found(),
        OrganizationTeacherApplicationListError::Connection(error) => super::db_connection_failed(
            "organization_teacher_applications_connection_failed",
            format!("organization_id={organization_id}"),
            error,
        ),
        OrganizationTeacherApplicationListError::Database(error) => super::logged_internal(
            "organization_teacher_applications_fetch_failed",
            &format!("organization_id={organization_id}"),
            "Failed to fetch organization teacher applications",
            error,
        ),
    }
}
