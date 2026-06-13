use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::teacher_application_service::{
    OrganizationTeacherApplicationsRequest, TeacherApplicationError,
};
use crate::utils::request_auth::authenticated_user;

pub(super) async fn get_organization_teacher_applications(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationTeacherApplicationsRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match crate::services::teacher_application_service::list_organization_applications(
        &mut conn,
        requester.user_id,
        organization_id,
        query.into_inner(),
    )
    .await
    {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(TeacherApplicationError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization teacher applications"),
        Err(TeacherApplicationError::InvalidInput(message)) => {
            HttpResponse::BadRequest().body(message)
        }
        Err(TeacherApplicationError::InvalidTransition(message)) => {
            HttpResponse::Conflict().body(message)
        }
        Err(TeacherApplicationError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(TeacherApplicationError::Database(error)) => {
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
