use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentStore,
};
use crate::domain::learning::enrollment::status::CourseJoinRequestStatus;

pub(super) const REQUEST_JOIN_COURSE: &str = "REQUEST_JOIN_COURSE";
pub(super) const JOIN_COURSE: &str = "JOIN_COURSE";
pub(super) const APPROVE_COURSE_JOIN_REQUESTS: &str = "APPROVE_COURSE_JOIN_REQUESTS";
pub(super) const MANAGE_COURSE_ENROLLMENTS: &str = "MANAGE_COURSE_ENROLLMENTS";

pub(super) async fn ensure_course_exists(
    store: &mut impl CourseEnrollmentStore,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    if store.course_exists(course_id).await? {
        Ok(())
    } else {
        Err(CourseEnrollmentError::NotFound)
    }
}

pub(super) async fn ensure_permission_any(
    store: &mut impl CourseEnrollmentStore,
    user_id: i32,
    course_id: i32,
    permissions: &[&str],
    denied_permission: &str,
) -> Result<(), CourseEnrollmentError> {
    for permission in permissions {
        if store
            .has_course_context_permission(user_id, course_id, permission)
            .await?
        {
            return Ok(());
        }
    }
    Err(CourseEnrollmentError::PermissionDenied(
        denied_permission.to_string(),
    ))
}

pub(super) fn normalize_join_decision(status: &str) -> Result<String, CourseEnrollmentError> {
    CourseJoinRequestStatus::normalize_decision(status)
        .map(|status| status.as_str().to_string())
        .map_err(|_| {
            CourseEnrollmentError::InvalidStatus(
                "unsupported course join decision status".to_string(),
            )
        })
}
