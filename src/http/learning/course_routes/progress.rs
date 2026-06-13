use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::course_service::{get_learner_progress, save_learner_progress};
use crate::utils::request_auth::authenticated_user_id;

use super::dto::SaveProgressRequest;
use super::support::learner_course_catalog_error_response;

pub(super) async fn save_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<SaveProgressRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match save_learner_progress(&mut conn, user_id, course_id, body.content_id).await {
        Ok(progress) => HttpResponse::Ok().json(progress),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

pub(super) async fn get_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_progress(&mut conn, user_id, course_id).await {
        Ok(progress) => HttpResponse::Ok().json(progress),
        Err(error) => learner_course_catalog_error_response(error),
    }
}
