use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::organization_service;
use crate::utils::request_auth::authenticated_user_id;

pub(super) async fn remove_organization_member_route(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (org_id, target_user_id) = path.into_inner();
    let _requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match organization_service::remove_organization_member(&pool, org_id, target_user_id).await {
        Ok(_) => HttpResponse::Ok().body("Member removed"),
        Err(msg) => {
            log::error!(
                "event=organization_member_remove_failed organization_id={} target_user_id={} error={}",
                org_id, target_user_id, msg
            );
            if msg.contains("User not found") {
                HttpResponse::NotFound().body(msg)
            } else {
                HttpResponse::InternalServerError().body("Failed to remove member")
            }
        }
    }
}
