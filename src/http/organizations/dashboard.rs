use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::organization_service::{self, OrganizationDashboardError};
use crate::utils::request_auth::authenticated_user;

pub(super) async fn get_organization_dashboard(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
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

    match organization_service::get_organization_dashboard(
        &mut conn,
        requester.user_id,
        organization_id,
    )
    .await
    {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(OrganizationDashboardError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization dashboard"),
        Err(OrganizationDashboardError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationDashboardError::Database(error)) => {
            log::error!(
                "event=organization_dashboard_fetch_failed organization_id={} error={:?}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization dashboard")
        }
        Err(OrganizationDashboardError::Reporting(error)) => {
            log::error!(
                "event=organization_dashboard_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization dashboard")
        }
    }
}
