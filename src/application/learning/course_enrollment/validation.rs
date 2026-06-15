use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentStore,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::learning::enrollment::status::CourseJoinRequestStatus;

pub(super) const REQUEST_JOIN_COURSE: Permissions = Permissions::REQUEST_JOIN_COURSE;
pub(super) const JOIN_COURSE: Permissions = Permissions::JOIN_COURSE;
pub(super) const APPROVE_COURSE_JOIN_REQUESTS: Permissions =
    Permissions::APPROVE_COURSE_JOIN_REQUESTS;
pub(super) const MANAGE_COURSE_ENROLLMENTS: Permissions = Permissions::MANAGE_COURSE_ENROLLMENTS;

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
    permissions: &[Permissions],
    denied_permission: Permissions,
) -> Result<(), CourseEnrollmentError> {
    for permission in permissions {
        let permission_name = permission.to_string();
        if store
            .has_course_context_permission(user_id, course_id, &permission_name)
            .await?
        {
            return Ok(());
        }
    }
    Err(CourseEnrollmentError::PermissionDenied(
        denied_permission.into(),
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
