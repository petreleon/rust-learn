use chrono::Utc;

use crate::application::learning::course_enrollment::validation::{
    ensure_course_exists, ensure_permission_any, normalize_join_decision,
    APPROVE_COURSE_JOIN_REQUESTS, JOIN_COURSE, MANAGE_COURSE_ENROLLMENTS, REQUEST_JOIN_COURSE,
};
use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentRemovalOutput, CourseEnrollmentStore,
    CourseJoinDecisionOutput, CourseJoinRequestOutput, DecideCourseJoinCommand,
    EnrollmentNotification, RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
use crate::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};

pub async fn request_course_join(
    store: &mut impl CourseEnrollmentStore,
    command: RequestCourseJoinCommand,
) -> Result<CourseJoinRequestOutput, CourseEnrollmentError> {
    ensure_course_exists(store, command.course_id).await?;
    ensure_permission_any(
        store,
        command.actor_user_id,
        command.course_id,
        &[REQUEST_JOIN_COURSE, JOIN_COURSE],
        REQUEST_JOIN_COURSE,
    )
    .await?;

    if store
        .has_student_role(command.actor_user_id, command.course_id)
        .await?
    {
        return Err(CourseEnrollmentError::InvalidStatus(
            "user is already enrolled in this course".to_string(),
        ));
    }

    if let Some(existing) = store
        .open_join_request(command.course_id, command.actor_user_id)
        .await?
    {
        return Ok(existing);
    }

    store
        .create_join_request(
            command.course_id,
            command.actor_user_id,
            COURSE_JOIN_STATUS_PENDING.to_string(),
        )
        .await
}

pub async fn decide_course_join_request(
    store: &mut impl CourseEnrollmentStore,
    command: DecideCourseJoinCommand,
) -> Result<CourseJoinDecisionOutput, CourseEnrollmentError> {
    let target_status = normalize_join_decision(&command.status)?;
    let existing = store
        .join_request(command.request_id)
        .await?
        .ok_or(CourseEnrollmentError::NotFound)?;

    if existing.course_id != command.course_id {
        return Err(CourseEnrollmentError::NotFound);
    }
    if existing.status == target_status {
        return Ok(CourseJoinDecisionOutput {
            join_request: existing,
            enrollment_notification: None,
        });
    }
    if !matches!(
        existing.status.as_str(),
        COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED
    ) {
        return Err(CourseEnrollmentError::InvalidStatus(
            "course join request has already been decided".to_string(),
        ));
    }

    ensure_permission_any(
        store,
        command.reviewer_user_id,
        command.course_id,
        &[APPROVE_COURSE_JOIN_REQUESTS, MANAGE_COURSE_ENROLLMENTS],
        APPROVE_COURSE_JOIN_REQUESTS,
    )
    .await?;

    let updated = store
        .update_join_request_decision(
            command.request_id,
            target_status.clone(),
            command.reviewer_user_id,
            command.decision_reason,
            Utc::now(),
        )
        .await?;
    let enrollment_notification = enrollment_notification(store, &updated, &target_status).await?;

    Ok(CourseJoinDecisionOutput {
        join_request: updated,
        enrollment_notification,
    })
}

pub async fn remove_course_enrollment(
    store: &mut impl CourseEnrollmentStore,
    command: RemoveCourseEnrollmentCommand,
) -> Result<CourseEnrollmentRemovalOutput, CourseEnrollmentError> {
    ensure_course_exists(store, command.course_id).await?;
    ensure_permission_any(
        store,
        command.actor_user_id,
        command.course_id,
        &[MANAGE_COURSE_ENROLLMENTS],
        MANAGE_COURSE_ENROLLMENTS,
    )
    .await?;

    if !store
        .remove_student_role(command.target_user_id, command.course_id)
        .await?
    {
        return Err(CourseEnrollmentError::NotFound);
    }

    Ok(CourseEnrollmentRemovalOutput {
        course_id: command.course_id,
        user_id: command.target_user_id,
        removed: true,
    })
}

async fn enrollment_notification(
    store: &mut impl CourseEnrollmentStore,
    request: &CourseJoinRequestOutput,
    target_status: &str,
) -> Result<Option<EnrollmentNotification>, CourseEnrollmentError> {
    if target_status != COURSE_JOIN_STATUS_APPROVED {
        return Ok(None);
    }

    store
        .assign_student_role_if_missing(request.requester_user_id, request.course_id)
        .await?;
    let course_title = store
        .course_title(request.course_id)
        .await?
        .unwrap_or_else(|| format!("course #{}", request.course_id));

    Ok(Some(EnrollmentNotification {
        target_user_id: request.requester_user_id,
        course_id: request.course_id,
        course_title,
    }))
}
