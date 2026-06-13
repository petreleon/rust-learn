use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::organization_service;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;

use super::dto::AssignRoleRequest;

pub(super) async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (org_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match organization_service::assign_role(&pool, requester_id, target_user_id, org_id, role_name)
        .await
    {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "organization",
                        Some(org_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=organization organization_id={} target_user_id={} error={:?}",
                        org_id,
                        target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(msg) => {
            if msg.contains("Hierarchy check failed") {
                HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.")
            } else if msg.contains("Role or User not found") {
                HttpResponse::BadRequest().body(msg)
            } else {
                log::error!(
                    "event=organization_role_assign_failed organization_id={} requester_user_id={} target_user_id={} role={} error={}",
                    org_id,
                    requester_id,
                    target_user_id,
                    role_name,
                    msg
                );
                HttpResponse::InternalServerError().body("Failed to assign role")
            }
        }
    }
}
