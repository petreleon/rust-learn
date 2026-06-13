use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db;
use crate::db::schema::{course_join_requests, courses};
use crate::models::course_join_request::COURSE_JOIN_STATUS_APPROVED;
use crate::services::course_enrollment_service::{
    decide_course_join_request as decide_course_join_request_for_actor,
    remove_course_enrollment as remove_course_enrollment_for_actor,
    request_course_join as request_course_join_for_actor, CourseJoinDecisionRequest,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user;

use super::support::course_enrollment_error_response;

async fn send_enrollment_notification_for_course(
    conn: &mut AsyncPgConnection,
    notifications: &NotificationsState,
    target_user_id: i32,
    course_id: i32,
) {
    let course_title = courses::table
        .find(course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await
        .unwrap_or_else(|_| format!("course #{}", course_id));

    if let Err(err) = notifications
        .send_enrollment_notification(target_user_id, course_id, course_title)
        .await
    {
        log::warn!(
            "event=notification_send_failed kind=enrollment course_id={} target_user_id={} error={:?}",
            course_id,
            target_user_id,
            err
        );
    }
}

pub(super) async fn request_course_join(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match request_course_join_for_actor(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(join_request) => HttpResponse::Created().json(join_request),
        Err(error) => course_enrollment_error_response(error),
    }
}

pub(super) async fn decide_course_join_request(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> impl Responder {
    let reviewer = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, request_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let status_before_decision = course_join_requests::table
        .find(request_id)
        .select(course_join_requests::status)
        .first::<String>(&mut conn)
        .await
        .optional()
        .ok()
        .flatten();

    match decide_course_join_request_for_actor(
        &mut conn,
        reviewer.user_id,
        course_id,
        request_id,
        body.into_inner(),
    )
    .await
    {
        Ok(join_request) => {
            if join_request.status == COURSE_JOIN_STATUS_APPROVED
                && status_before_decision.as_deref() != Some(COURSE_JOIN_STATUS_APPROVED)
            {
                if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                    send_enrollment_notification_for_course(
                        &mut conn,
                        notifications,
                        join_request.requester_user_id,
                        join_request.course_id,
                    )
                    .await;
                }
            }

            HttpResponse::Ok().json(join_request)
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

pub(super) async fn remove_course_enrollment(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let actor = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, target_user_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match remove_course_enrollment_for_actor(&mut conn, actor.user_id, course_id, target_user_id)
        .await
    {
        Ok(removal) => HttpResponse::Ok().json(removal),
        Err(error) => course_enrollment_error_response(error),
    }
}
