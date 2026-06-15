use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementUseCase, OrganizationOutput,
    OrganizationUpdateCommand,
};
use crate::http::errors::ApiError;

use super::dto::{CreateOrganizationRequest, UpdateOrganizationRequest};
use super::errors::organization_management_error;
use super::organization_dto::OrganizationResponse;

pub(super) async fn list_organizations(
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> Result<web::Json<Vec<OrganizationResponse>>, ApiError> {
    use_case
        .list_organizations()
        .await
        .map(organization_responses)
        .map(web::Json)
        .map_err(|error| {
            organization_management_error(
                error,
                None,
                "organization_list_failed",
                "Failed to load organizations",
            )
        })
}

pub(super) async fn get_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> Result<web::Json<OrganizationResponse>, ApiError> {
    let organization_id = path.into_inner();
    use_case
        .get_organization(organization_id)
        .await
        .map(OrganizationResponse::from)
        .map(web::Json)
        .map_err(|error| {
            organization_management_error(
                error,
                Some(organization_id),
                "organization_fetch_failed",
                "Failed to fetch organization",
            )
        })
}

pub(super) async fn create_organization(
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
    req: web::Json<CreateOrganizationRequest>,
) -> Result<(web::Json<OrganizationResponse>, StatusCode), ApiError> {
    let command = OrganizationCreateCommand {
        name: req.name.clone(),
        website_link: req.website_link.clone(),
        profile_url: req.profile_url.clone(),
        course_ids: req.course_ids.clone(),
    };

    use_case
        .create_organization(command)
        .await
        .map(OrganizationResponse::from)
        .map(web::Json)
        .map(|body| (body, StatusCode::CREATED))
        .map_err(|error| {
            organization_management_error(
                error,
                None,
                "organization_create_failed",
                "Failed to create organization",
            )
        })
}

pub(super) async fn update_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
    req: web::Json<UpdateOrganizationRequest>,
) -> Result<web::Json<OrganizationResponse>, ApiError> {
    let organization_id = path.into_inner();
    let update = req.into_inner();
    let command = OrganizationUpdateCommand {
        organization_id,
        name: update.name,
        website_link: update.website_link,
        profile_url: update.profile_url,
    };

    use_case
        .update_organization(command)
        .await
        .map(OrganizationResponse::from)
        .map(web::Json)
        .map_err(|error| {
            organization_management_error(
                error,
                Some(organization_id),
                "organization_update_failed",
                "Failed to update organization",
            )
        })
}

pub(super) async fn delete_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let organization_id = path.into_inner();
    use_case
        .delete_organization(organization_id)
        .await
        .map(|_| ("Organization deleted", StatusCode::OK))
        .map_err(|error| {
            organization_management_error(
                error,
                Some(organization_id),
                "organization_delete_failed",
                "Failed to delete organization",
            )
        })
}

fn organization_responses(organizations: Vec<OrganizationOutput>) -> Vec<OrganizationResponse> {
    organizations.into_iter().map(Into::into).collect()
}
