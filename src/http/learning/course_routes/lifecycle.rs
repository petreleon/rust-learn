use std::sync::Arc;

use actix_web::web;

use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{CourseLifecycleUpdateRequest, CourseResponse};

use super::errors::lifecycle_error;

pub(super) async fn update_course_lifecycle(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseLifecycleUseCase>>,
    body: web::Json<CourseLifecycleUpdateRequest>,
) -> Result<web::Json<CourseResponse>, ApiError> {
    let command = CourseLifecycleCommand {
        actor_user_id: requester.user_id(),
        course_id: path.into_inner(),
        status: body.into_inner().status,
    };

    use_case
        .update_course_lifecycle(command)
        .await
        .map(CourseResponse::from)
        .map(web::Json)
        .map_err(lifecycle_error)
}
