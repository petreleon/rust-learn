use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListQuery, OrganizationCourseListUseCase,
};
use crate::utils::request_auth::authenticated_user;

use super::course_dto::OrganizationCourseListResponse;
use super::dto::OrganizationCourseListParams;

pub(super) async fn get_organization_courses(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationCourseListUseCase>>,
    query: web::Query<OrganizationCourseListParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();

    let list_query = OrganizationCourseListQuery::new(
        requester.user_id,
        organization_id,
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match use_case.list_organization_courses(list_query).await {
        Ok(courses) => HttpResponse::Ok().json(OrganizationCourseListResponse::from(courses)),
        Err(OrganizationCourseListError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization courses"),
        Err(OrganizationCourseListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationCourseListError::Connection(error)) => {
            log::error!(
                "event=organization_courses_connection_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(OrganizationCourseListError::Database(error)) => {
            log::error!(
                "event=organization_courses_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization courses")
        }
    }
}
