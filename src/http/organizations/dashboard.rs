use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardQuery, OrganizationDashboardUseCase,
};
use crate::utils::request_auth::authenticated_user;

use super::dashboard_dto::OrganizationDashboardResponse;

pub(super) async fn get_organization_dashboard(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationDashboardUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let query = OrganizationDashboardQuery {
        actor_user_id: requester.user_id,
        organization_id,
    };

    match use_case.get_organization_dashboard(query).await {
        Ok(dashboard) => HttpResponse::Ok().json(OrganizationDashboardResponse::from(dashboard)),
        Err(OrganizationDashboardError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization dashboard"),
        Err(OrganizationDashboardError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationDashboardError::Connection(error)) => {
            log::error!(
                "event=organization_dashboard_connection_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(OrganizationDashboardError::Database(error)) => {
            log::error!(
                "event=organization_dashboard_fetch_failed organization_id={} error={}",
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
