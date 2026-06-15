use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentUseCase, RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
use crate::application::notifications::delivery::{
    EnrollmentNotificationCommand, NotificationDeliveryUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::{
    CourseEnrollmentRemovalResponse, CourseJoinDecisionRequest, CourseJoinRequestResponse,
};

use super::errors::course_enrollment_error;

async fn send_enrollment_notification(
    notifications: &Arc<dyn NotificationDeliveryUseCase>,
    notification: crate::application::learning::course_enrollment::EnrollmentNotification,
) {
    if let Err(err) = notifications
        .send_enrollment(EnrollmentNotificationCommand {
            target_user_id: notification.target_user_id,
            course_id: notification.course_id,
            course_title: notification.course_title,
        })
        .await
    {
        log::warn!(
            "event=notification_send_failed kind=enrollment course_id={} target_user_id={} error={}",
            notification.course_id,
            notification.target_user_id,
            err.message()
        );
    }
}

pub(super) async fn request_course_join(
    requester: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
) -> Result<(web::Json<CourseJoinRequestResponse>, StatusCode), ApiError> {
    let requester_user_id = requester.into_inner();

    use_case
        .request_course_join(RequestCourseJoinCommand {
            actor_user_id: requester_user_id,
            course_id: path.into_inner(),
        })
        .await
        .map(CourseJoinRequestResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(course_enrollment_error)
}

pub(super) async fn decide_course_join_request(
    reviewer: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> Result<web::Json<CourseJoinRequestResponse>, ApiError> {
    let reviewer_user_id = reviewer.into_inner();
    let (course_id, request_id) = path.into_inner();

    let output = use_case
        .decide_course_join_request(body.into_inner().into_command(
            reviewer_user_id,
            course_id,
            request_id,
        ))
        .await
        .map_err(course_enrollment_error)?;

    if let Some(notification) = output.enrollment_notification {
        if let Some(notifications) = notifications {
            send_enrollment_notification(&notifications, notification).await;
        }
    }

    Ok(web::Json(CourseJoinRequestResponse::from(
        output.join_request,
    )))
}

pub(super) async fn remove_course_enrollment(
    actor: AuthUserId,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
) -> Result<web::Json<CourseEnrollmentRemovalResponse>, ApiError> {
    let actor_user_id = actor.into_inner();
    let (course_id, target_user_id) = path.into_inner();

    use_case
        .remove_course_enrollment(RemoveCourseEnrollmentCommand {
            actor_user_id,
            course_id,
            target_user_id,
        })
        .await
        .map(CourseEnrollmentRemovalResponse::from)
        .map(web::Json)
        .map_err(course_enrollment_error)
}
