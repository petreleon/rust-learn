use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListQuery,
    OrganizationTeacherApplicationListUseCase,
};
use crate::http::extractors::request_auth::authenticated_user;

use super::dto::OrganizationTeacherApplicationsParams;
use super::teacher_application_dto::OrganizationTeacherApplicationsResponse;

pub(super) async fn get_organization_teacher_applications(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationTeacherApplicationListUseCase>>,
    query: web::Query<OrganizationTeacherApplicationsParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();

    let list_query = OrganizationTeacherApplicationListQuery {
        actor_user_id: requester.user_id,
        organization_id,
        status: query.status.clone(),
        search: query.search.clone(),
        limit: query.limit,
        offset: query.offset,
    };

    match use_case
        .list_organization_teacher_applications(list_query)
        .await
    {
        Ok(applications) => {
            HttpResponse::Ok().json(OrganizationTeacherApplicationsResponse::from(applications))
        }
        Err(OrganizationTeacherApplicationListError::PermissionDenied(_)) => {
            HttpResponse::Forbidden()
                .body("User does not have permission to view organization teacher applications")
        }
        Err(OrganizationTeacherApplicationListError::InvalidInput(message)) => {
            HttpResponse::BadRequest().body(message)
        }
        Err(OrganizationTeacherApplicationListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationTeacherApplicationListError::Connection(error)) => {
            log::error!(
                "event=organization_teacher_applications_connection_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(OrganizationTeacherApplicationListError::Database(error)) => {
            log::error!(
                "event=organization_teacher_applications_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError()
                .body("Failed to fetch organization teacher applications")
        }
    }
}
