use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::learning::create_course::CourseCreationUseCase;
use crate::application::learning::delete_course::{
    CourseDeletionError, CourseDeletionOutcome, CourseDeletionUseCase,
};
use crate::application::learning::update_course::CourseUpdateUseCase;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{CourseResponse, CourseUpdateRequest, CreateCourseRequest};

use super::support::{course_creation_error_response, course_update_error_response};

pub(super) async fn create_course(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn CourseCreationUseCase>>,
    body: web::Json<CreateCourseRequest>,
) -> impl Responder {
    let command = body.into_inner().into_command(requester.user_id());

    match use_case.create_course(command).await {
        Ok(course) => HttpResponse::Created().json(CourseResponse::from(course)),
        Err(error) => course_creation_error_response(error),
    }
}

pub(super) async fn update_course(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseUpdateUseCase>>,
    body: web::Json<CourseUpdateRequest>,
) -> impl Responder {
    let course_id = path.into_inner();

    let command = body
        .into_inner()
        .into_command(requester.user_id(), course_id);

    match use_case.update_course(command).await {
        Ok(course) => HttpResponse::Ok().json(CourseResponse::from(course)),
        Err(error) => course_update_error_response(error),
    }
}

pub(super) async fn delete_course(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseDeletionUseCase>>,
) -> impl Responder {
    let course_id = path.into_inner();

    match use_case.delete_course(course_id).await {
        Ok(CourseDeletionOutcome::Deleted) => HttpResponse::Ok().body("Course deleted"),
        Ok(CourseDeletionOutcome::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(CourseDeletionError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(CourseDeletionError::Database(message)) => {
            log::error!(
                "event=course_delete_failed course_id={} error={}",
                course_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to delete course")
        }
    }
}
