use crate::db;
use crate::models::delegated_permission::GrantDelegatedPermissionRequest;
use crate::services::delegated_permission_service::{
    self, DelegatedPermissionError, ListDelegatedPermissionsRequest,
};
use crate::utils::request_auth::authenticated_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RevokeDelegatedPermissionRequest {
    revoke_reason: Option<String>,
}

fn delegated_permission_error_response(error: DelegatedPermissionError) -> HttpResponse {
    match error {
        DelegatedPermissionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have delegated-permission access")
        }
        DelegatedPermissionError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        DelegatedPermissionError::NotFound => {
            HttpResponse::NotFound().body("Delegated permission not found")
        }
        DelegatedPermissionError::Database(message) => {
            log::error!("event=delegated_permission_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process delegated permission")
        }
    }
}

async fn grant_delegated_permission(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<GrantDelegatedPermissionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match delegated_permission_service::grant_delegated_permission(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(delegation) => HttpResponse::Created().json(delegation),
        Err(error) => delegated_permission_error_response(error),
    }
}

async fn list_delegated_permissions(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListDelegatedPermissionsRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match delegated_permission_service::list_delegated_permissions(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(delegations) => HttpResponse::Ok().json(delegations),
        Err(error) => delegated_permission_error_response(error),
    }
}

async fn revoke_delegated_permission(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
    body: web::Json<RevokeDelegatedPermissionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match delegated_permission_service::revoke_delegated_permission(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner().revoke_reason,
    )
    .await
    {
        Ok(delegation) => HttpResponse::Ok().json(delegation),
        Err(error) => delegated_permission_error_response(error),
    }
}

pub fn delegated_permission_scope() -> actix_web::Scope {
    web::scope("/delegated-permissions")
        .service(
            web::resource("")
                .route(web::post().to(grant_delegated_permission))
                .route(web::get().to(list_delegated_permissions)),
        )
        .service(web::resource("/{id}/revoke").route(web::put().to(revoke_delegated_permission)))
}
