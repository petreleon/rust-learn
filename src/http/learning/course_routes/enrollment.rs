use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentUseCase, RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
use crate::http::learning::dto::{
    CourseEnrollmentRemovalResponse, CourseJoinDecisionRequest, CourseJoinRequestResponse,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

use super::support::course_enrollment_error_response;

async fn send_enrollment_notification(
    notifications: &NotificationsState,
    notification: crate::application::learning::course_enrollment::EnrollmentNotification,
) {
    if let Err(err) = notifications
        .send_enrollment_notification(
            notification.target_user_id,
            notification.course_id,
            notification.course_title,
        )
        .await
    {
        log::warn!(
            "event=notification_send_failed kind=enrollment course_id={} target_user_id={} error={:?}",
            notification.course_id,
            notification.target_user_id,
            err
        );
    }
}

pub(super) async fn request_course_join(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
) -> impl Responder {
    let requester_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

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
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> impl Responder {
    let reviewer_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };
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
                if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                    send_enrollment_notification(notifications, notification).await;
                }
            }
            HttpResponse::Ok().json(CourseJoinRequestResponse::from(output.join_request))
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

pub(super) async fn remove_course_enrollment(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn CourseEnrollmentUseCase>>,
) -> impl Responder {
    let actor_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };
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
