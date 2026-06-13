use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::repositories::course_repository::assign_role_to_user_in_course;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

use super::dto::AssignRoleRequest;

pub(super) async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (course_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match assign_role_to_user_in_course(
        &mut conn,
        requester_id,
        target_user_id,
        course_id,
        role_name,
    )
    .await
    {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "course",
                        Some(course_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=course course_id={} target_user_id={} error={:?}",
                        course_id,
                        target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(diesel::result::Error::RollbackTransaction) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(diesel::result::Error::NotFound) => HttpResponse::BadRequest().body("Role or User not found"),
        Err(e) => {
            log::error!(
                "event=course_role_assign_failed requester_user_id={} target_user_id={} course_id={} role={} error={}",
                requester_id,
                target_user_id,
                course_id,
                role_name,
                e
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}
