use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleUseCase,
};
use crate::http::extractors::request_auth::authenticated_user;
use crate::http::learning::dto::{CourseLifecycleUpdateRequest, CourseResponse};

use super::support::lifecycle_error_response;

pub(super) async fn update_course_lifecycle(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseLifecycleUseCase>>,
    body: web::Json<CourseLifecycleUpdateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let command = CourseLifecycleCommand {
        actor_user_id: requester.user_id,
        course_id: path.into_inner(),
        status: body.into_inner().status,
    };

    match use_case.update_course_lifecycle(command).await {
        Ok(course) => HttpResponse::Ok().json(CourseResponse::from(course)),
        Err(error) => lifecycle_error_response(error),
    }
}
