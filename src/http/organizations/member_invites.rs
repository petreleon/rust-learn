use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::models::user::User;
use crate::services::organization_service;
use crate::utils::request_auth::authenticated_user_id;

use super::dto::AddMemberRequest;

pub(super) async fn add_member_by_email_route(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<AddMemberRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let org_id = path.into_inner();
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let target_user = match User::find_by_email(body.email.trim(), &mut conn).await {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("User not found by email")
        }
        Err(e) => {
            log::error!(
                "event=org_member_add_user_lookup_failed email={} error={}",
                body.email,
                e
            );
            return HttpResponse::InternalServerError().body("Failed to look up user");
        }
    };

    let role_name = body.role_name.as_deref().unwrap_or("STUDENT");

    match organization_service::assign_role(&pool, requester_id, target_user.id, org_id, role_name)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": target_user.id,
            "name": target_user.name,
            "email": target_user.email,
            "role": role_name,
        })),
        Err(msg) => {
            log::error!(
                "event=org_member_add_failed org_id={} user_id={} error={}",
                org_id,
                target_user.id,
                msg
            );
            if msg.contains("Hierarchy") {
                HttpResponse::Forbidden().body(msg)
            } else {
                HttpResponse::InternalServerError().body("Failed to add member")
            }
        }
    }
}
