use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::db::schema::users;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::user::User;
use crate::repositories::platform_repository::{
    assign_role_to_user_with_hierarchy, user_permission_platform_request,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::{authenticated_user, authenticated_user_id};
use actix_web::{web, HttpRequest};
use actix_web::{HttpResponse, Responder};
use diesel::{BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

// GET /user -> list users for callers with VIEW_USER.
async fn list_users(
    pool: web::Data<db::DbPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let search = query.get("search").map(|s| s.trim()).filter(|s| !s.is_empty());
    let result = if let Some(term) = search {
        let pattern = format!("%{}%", term);
        let rows = users::table
            .filter(
                users::name
                    .ilike(&pattern)
                    .or(users::email.ilike(&pattern)),
            )
            .load::<User>(&mut conn)
            .await;
        rows
    } else {
        User::find_all(&mut conn).await
    };

    match result {
        Ok(user_list) => {
            let users_json: Vec<_> = user_list
                .iter()
                .map(|u| {
                    json!({
                        "id": u.id,
                        "name": u.name,
                        "email": u.email,
                        "date_of_birth": u.date_of_birth.map(|d| d.to_string()),
                        "created_at": u.created_at.to_string(),
                        "kyc_verified": u.kyc_verified,
                        "email_verified": u.email_verified,
                    })
                })
                .collect();
            HttpResponse::Ok().json(json!({ "users": users_json }))
        }
        Err(e) => {
            log::error!("event=user_list_failed error={}", e);
            HttpResponse::InternalServerError().body("Failed to load users")
        }
    }
}

// GET /user/{id} -> users can read themselves; VIEW_USER can read anyone.
async fn get_user(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    if requester.user_id != user_id {
        match user_permission_platform_request(
            &mut conn,
            requester.user_id,
            &Permissions::VIEW_USER.to_string(),
        )
        .await
        {
            Ok(true) => {}
            Ok(false) => {
                return HttpResponse::Forbidden().body("User does not have the required permission")
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to check user permission")
            }
        }
    }

    let result = User::find_by_id(user_id, &mut conn).await;

    match result {
        Ok(u) => HttpResponse::Ok().json(json!({
            "id": u.id,
            "name": u.name,
            "email": u.email,
            "date_of_birth": u.date_of_birth.map(|d| d.to_string()),
            "created_at": u.created_at.to_string(),
            "kyc_verified": u.kyc_verified,
            "email_verified": u.email_verified,
        })),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            log::error!("event=user_fetch_failed user_id={} error={}", user_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

// POST /user/{id}/role -> assign role to user
async fn assign_role(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let target_user_id = path.into_inner();
    let role_name = &body.role_name;

    // 1. Get DB connection
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    // 2. Identify Requester from JWT
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match assign_role_to_user_with_hierarchy(&mut conn, requester_id, target_user_id, role_name)
        .await
    {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "platform",
                        None,
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=platform target_user_id={} error={:?}",
                        target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(diesel::result::Error::RollbackTransaction) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::BadRequest().body(format!("Role '{}' not found", role_name))
        }
        Err(e) => {
            log::error!(
                "event=platform_role_assign_failed requester_user_id={} target_user_id={} role={} error={}",
                requester_id,
                target_user_id,
                role_name,
                e
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(
            web::resource("").route(web::get().to(list_users).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_USER.to_string()),
            )),
        )
        .service(web::resource("/{id}").route(web::get().to(get_user)))
        .service(
            web::resource("/{id}/role").route(web::post().to(assign_role).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::ASSIGN_ROLES_TO_USER.to_string(),
                ),
            )),
        )
}
