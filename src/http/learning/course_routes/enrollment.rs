use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentUseCase, RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
use crate::application::notifications::delivery::{
    EnrollmentNotificationCommand, NotificationDeliveryUseCase,
};
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::{
    CourseEnrollmentRemovalResponse, CourseJoinDecisionRequest, CourseJoinRequestResponse,
};

use super::support::course_enrollment_error_response;

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
) -> impl Responder {
    let requester_user_id = requester.into_inner();

    match use_case
        .request_course_join(RequestCourseJoinCommand {
            actor_user_id: requester_user_id,
            course_id: path.into_inner(),
        })
        .await
    {
        Ok(join_request) => {
            HttpResponse::Created().json(CourseJoinRequestResponse::from(join_request))
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

pub(super) async fn decide_course_join_request(
    reviewer: AuthUserId,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> impl Responder {
    let reviewer_user_id = reviewer.into_inner();
    let (course_id, request_id) = path.into_inner();

    match use_case
        .decide_course_join_request(body.into_inner().into_command(
            reviewer_user_id,
            course_id,
            request_id,
        ))
        .await
    {
        Ok(output) => {
            if let Some(notification) = output.enrollment_notification {
                if let Some(notifications) = notifications {
                    send_enrollment_notification(&notifications, notification).await;
                }
            }
            HttpResponse::Ok().json(CourseJoinRequestResponse::from(output.join_request))
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

pub(super) async fn remove_course_enrollment(
    actor: AuthUserId,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
) -> impl Responder {
    let actor_user_id = actor.into_inner();
    let (course_id, target_user_id) = path.into_inner();

    match use_case
        .remove_course_enrollment(RemoveCourseEnrollmentCommand {
            actor_user_id,
            course_id,
            target_user_id,
        })
        .await
    {
        Ok(removal) => HttpResponse::Ok().json(CourseEnrollmentRemovalResponse::from(removal)),
        Err(error) => course_enrollment_error_response(error),
    }
}
