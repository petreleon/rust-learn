use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::course_service::CourseLifecycleUpdateRequest;
use crate::utils::request_auth::authenticated_user;

use super::support::lifecycle_error_response;

pub(super) async fn update_course_lifecycle(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseLifecycleUpdateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match crate::services::course_service::update_course_lifecycle(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => lifecycle_error_response(error),
    }
}
