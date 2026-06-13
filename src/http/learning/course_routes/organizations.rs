use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::learning::list_course_organizations::{
    CourseOrganizationReadError, CourseOrganizationsUseCase,
};
use crate::http::learning::dto::CourseOrganizationResponse;

pub(super) async fn get_course_organizations(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseOrganizationsUseCase>>,
) -> impl Responder {
    let course_id = path.into_inner();

    match use_case.list_course_organizations(course_id).await {
        Ok(organizations) => HttpResponse::Ok().json(
            organizations
                .into_iter()
                .map(CourseOrganizationResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(error) => course_organizations_error_response(course_id, error),
    }
}

fn course_organizations_error_response(
    course_id: i32,
    error: CourseOrganizationReadError,
) -> HttpResponse {
    match error {
        CourseOrganizationReadError::Connection(_) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseOrganizationReadError::Database(message) => {
            log::error!(
                "event=course_organizations_fetch_failed course_id={} error={}",
                course_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch course organizations")
        }
    }
}
