use crate::application::learning::discover_courses::{CourseDiscoveryError, CourseDiscoveryQuery};
use crate::application::learning::get_course::CourseReadError;
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError as LearnerCourseReadError;
use crate::application::learning::learner_progress::LearnerProgressError;
use crate::application::learning::list_course_organizations::CourseOrganizationReadError;
use crate::http::errors::ApiError;

pub(in crate::http::learning::course_routes) fn learner_course_read_error(
    error: LearnerCourseReadError,
) -> ApiError {
    match error {
        LearnerCourseReadError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to view course content")
        }
        LearnerCourseReadError::NotFound => super::course_not_found(),
        LearnerCourseReadError::Connection(message) => {
            super::db_connection_failed("learner_course_read_connection_failed", message)
        }
        LearnerCourseReadError::Database(message) => super::logged_internal(
            "learner_course_read_failed",
            "",
            "Failed to load course catalog",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn learner_progress_error(
    error: LearnerProgressError,
) -> ApiError {
    match error {
        LearnerProgressError::PermissionDenied(_) => {
            super::permission_denied("User does not have permission to view course content")
        }
        LearnerProgressError::NotFound => super::course_not_found(),
        LearnerProgressError::Connection(message) => {
            super::db_connection_failed("learner_progress_connection_failed", message)
        }
        LearnerProgressError::Database(message) => super::logged_internal(
            "learner_progress_failed",
            "",
            "Failed to load course catalog",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_discovery_error(
    query: &CourseDiscoveryQuery,
    error: CourseDiscoveryError,
) -> ApiError {
    match error {
        CourseDiscoveryError::Connection(message) => {
            super::db_connection_failed("course_list_connection_failed", message)
        }
        CourseDiscoveryError::Database(message) => super::logged_internal(
            "course_list_failed",
            &format!(
                "search={:?} organization_id={:?} limit={} offset={}",
                query.search, query.organization_id, query.limit, query.offset
            ),
            "Failed to load courses",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_read_error(
    course_id: i32,
    error: CourseReadError,
) -> ApiError {
    match error {
        CourseReadError::NotFound => super::course_not_found(),
        CourseReadError::Connection(message) => {
            super::db_connection_failed("course_fetch_connection_failed", message)
        }
        CourseReadError::Database(message) => super::logged_internal(
            "course_fetch_failed",
            &format!("course_id={course_id}"),
            "Failed to fetch course",
            message,
        ),
    }
}

pub(in crate::http::learning::course_routes) fn course_organizations_error(
    course_id: i32,
    error: CourseOrganizationReadError,
) -> ApiError {
    match error {
        CourseOrganizationReadError::Connection(message) => {
            super::db_connection_failed("course_organizations_connection_failed", message)
        }
        CourseOrganizationReadError::Database(message) => super::logged_internal(
            "course_organizations_fetch_failed",
            &format!("course_id={course_id}"),
            "Failed to fetch course organizations",
            message,
        ),
    }
}
