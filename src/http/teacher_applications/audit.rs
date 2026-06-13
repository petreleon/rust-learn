use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::http::teacher_applications::support::service_error_response;
use crate::services::teacher_application_service;
use crate::utils::request_auth::authenticated_user;

pub(super) async fn list_audit_events(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_audit_events(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(error) => service_error_response(error),
    }
}
