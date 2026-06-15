use std::sync::Arc;

use actix_web::web;

use crate::application::learning::learner_progress::LearnerProgressUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::{LearnerProgressResponse, SaveProgressRequest};

use super::errors::learner_progress_error;

pub(super) async fn save_learner_progress_route(
    user: AuthUserId,
    path: web::Path<i32>,
    body: web::Json<SaveProgressRequest>,
    use_case: web::Data<Arc<dyn LearnerProgressUseCase>>,
) -> Result<web::Json<LearnerProgressResponse>, ApiError> {
    let user_id = user.into_inner();
    let course_id = path.into_inner();
    let command = body.into_inner().into_command(user_id, course_id);

    use_case
        .save_progress(command)
        .await
        .map(LearnerProgressResponse::from)
        .map(web::Json)
        .map_err(learner_progress_error)
}

pub(super) async fn get_learner_progress_route(
    user: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn LearnerProgressUseCase>>,
) -> Result<web::Json<Option<LearnerProgressResponse>>, ApiError> {
    let user_id = user.into_inner();
    let course_id = path.into_inner();

    use_case
        .get_progress(user_id, course_id)
        .await
        .map(|progress| progress.map(LearnerProgressResponse::from))
        .map(web::Json)
        .map_err(learner_progress_error)
}
