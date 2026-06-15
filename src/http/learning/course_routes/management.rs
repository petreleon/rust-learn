use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::learning::create_course::CourseCreationUseCase;
use crate::application::learning::delete_course::{CourseDeletionOutcome, CourseDeletionUseCase};
use crate::application::learning::update_course::CourseUpdateUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{CourseResponse, CourseUpdateRequest, CreateCourseRequest};

use super::errors::{course_creation_error, course_deletion_error, course_update_error};

pub(super) async fn create_course(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn CourseCreationUseCase>>,
    body: web::Json<CreateCourseRequest>,
) -> Result<(web::Json<CourseResponse>, StatusCode), ApiError> {
    let command = body.into_inner().into_command(requester.user_id());

    use_case
        .create_course(command)
        .await
        .map(CourseResponse::from)
        .map(web::Json)
        .map(|response| (response, StatusCode::CREATED))
        .map_err(course_creation_error)
}

pub(super) async fn update_course(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseUpdateUseCase>>,
    body: web::Json<CourseUpdateRequest>,
) -> Result<web::Json<CourseResponse>, ApiError> {
    let course_id = path.into_inner();

    let command = body
        .into_inner()
        .into_command(requester.user_id(), course_id);

    use_case
        .update_course(command)
        .await
        .map(CourseResponse::from)
        .map(web::Json)
        .map_err(course_update_error)
}

pub(super) async fn delete_course(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseDeletionUseCase>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let course_id = path.into_inner();

    match use_case.delete_course(course_id).await {
        Ok(CourseDeletionOutcome::Deleted) => Ok(("Course deleted", StatusCode::OK)),
        Ok(CourseDeletionOutcome::NotFound) => Err(super::errors::course_read_error(
            course_id,
            crate::application::learning::get_course::CourseReadError::NotFound,
        )),
        Err(error) => Err(course_deletion_error(course_id, error)),
    }
}
