use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentStore,
};

pub(super) const REQUEST_JOIN_COURSE: &str = "REQUEST_JOIN_COURSE";
pub(super) const JOIN_COURSE: &str = "JOIN_COURSE";
pub(super) const APPROVE_COURSE_JOIN_REQUESTS: &str = "APPROVE_COURSE_JOIN_REQUESTS";
pub(super) const MANAGE_COURSE_ENROLLMENTS: &str = "MANAGE_COURSE_ENROLLMENTS";
pub(super) const COURSE_JOIN_STATUS_PENDING: &str = "pending";
pub(super) const COURSE_JOIN_STATUS_WAITLISTED: &str = "waitlisted";
pub(super) const COURSE_JOIN_STATUS_APPROVED: &str = "approved";
const COURSE_JOIN_STATUS_REJECTED: &str = "rejected";

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
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        COURSE_JOIN_STATUS_APPROVED
        | COURSE_JOIN_STATUS_REJECTED
        | COURSE_JOIN_STATUS_WAITLISTED => Ok(normalized),
        _ => Err(CourseEnrollmentError::InvalidStatus(
            "unsupported course join decision status".to_string(),
        )),
    }
}
