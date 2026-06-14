use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::learning::learner_progress::LearnerProgressUseCase;
use crate::http::extractors::request_auth::authenticated_user_id;
use crate::http::learning::dto::{LearnerProgressResponse, SaveProgressRequest};

use super::support::learner_progress_error_response;

pub(super) async fn save_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<SaveProgressRequest>,
    use_case: web::Data<Arc<dyn LearnerProgressUseCase>>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let command = body.into_inner().into_command(user_id, course_id);

    match use_case.save_progress(command).await {
        Ok(progress) => HttpResponse::Ok().json(LearnerProgressResponse::from(progress)),
        Err(error) => learner_progress_error_response(error),
    }
}

pub(super) async fn get_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerProgressUseCase>>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();

    match use_case.get_progress(user_id, course_id).await {
        Ok(progress) => HttpResponse::Ok().json(progress.map(LearnerProgressResponse::from)),
        Err(error) => learner_progress_error_response(error),
    }
}
