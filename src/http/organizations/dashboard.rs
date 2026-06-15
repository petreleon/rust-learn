use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardQuery, OrganizationDashboardUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::dashboard_dto::OrganizationDashboardResponse;
use super::errors::organization_dashboard_error;

pub(super) async fn get_organization_dashboard(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationDashboardUseCase>>,
) -> Result<web::Json<OrganizationDashboardResponse>, ApiError> {
    let organization_id = path.into_inner();
    let query = OrganizationDashboardQuery {
        actor_user_id: requester.user_id(),
        organization_id,
    };

    use_case
        .get_organization_dashboard(query)
        .await
        .map(OrganizationDashboardResponse::from)
        .map(web::Json)
        .map_err(|error| organization_dashboard_error(organization_id, error))
}
