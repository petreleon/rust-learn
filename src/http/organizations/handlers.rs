use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementUseCase,
    OrganizationOutput, OrganizationUpdateCommand,
};

use super::dto::{CreateOrganizationRequest, UpdateOrganizationRequest};
use super::organization_dto::OrganizationResponse;

pub(super) async fn list_organizations(
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> impl Responder {
    match use_case.list_organizations().await {
        Ok(organizations) => HttpResponse::Ok().json(organization_responses(organizations)),
        Err(error) => organization_management_error_response(
            error,
            None,
            "organization_list_failed",
            "Failed to load organizations",
        ),
    }
}

pub(super) async fn get_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    match use_case.get_organization(organization_id).await {
        Ok(organization) => HttpResponse::Ok().json(OrganizationResponse::from(organization)),
        Err(error) => organization_management_error_response(
            error,
            Some(organization_id),
            "organization_fetch_failed",
            "Failed to fetch organization",
        ),
    }
}

pub(super) async fn create_organization(
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
    req: web::Json<CreateOrganizationRequest>,
) -> impl Responder {
    let command = OrganizationCreateCommand {
        name: req.name.clone(),
        website_link: req.website_link.clone(),
        profile_url: req.profile_url.clone(),
        course_ids: req.course_ids.clone(),
    };

    match use_case.create_organization(command).await {
        Ok(organization) => HttpResponse::Created().json(OrganizationResponse::from(organization)),
        Err(error) => organization_management_error_response(
            error,
            None,
            "organization_create_failed",
            "Failed to create organization",
        ),
    }
}

pub(super) async fn update_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
    req: web::Json<UpdateOrganizationRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let update = req.into_inner();
    let command = OrganizationUpdateCommand {
        organization_id,
        name: update.name,
        website_link: update.website_link,
        profile_url: update.profile_url,
    };

    match use_case.update_organization(command).await {
        Ok(organization) => HttpResponse::Ok().json(OrganizationResponse::from(organization)),
        Err(error) => organization_management_error_response(
            error,
            Some(organization_id),
            "organization_update_failed",
            "Failed to update organization",
        ),
    }
}

pub(super) async fn delete_organization(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationManagementUseCase>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    match use_case.delete_organization(organization_id).await {
        Ok(_) => HttpResponse::Ok().body("Organization deleted"),
        Err(error) => organization_management_error_response(
            error,
            Some(organization_id),
            "organization_delete_failed",
            "Failed to delete organization",
        ),
    }
}

fn organization_management_error_response(
    error: OrganizationManagementError,
    organization_id: Option<i32>,
    event: &str,
    response_body: &'static str,
) -> HttpResponse {
    match error {
        OrganizationManagementError::NotFound => {
            HttpResponse::NotFound().body("Organization not found")
        }
        OrganizationManagementError::Connection(error)
        | OrganizationManagementError::Database(error) => {
            if let Some(organization_id) = organization_id {
                log::error!(
                    "event={} organization_id={} error={}",
                    event,
                    organization_id,
                    error
                );
            } else {
                log::error!("event={} error={}", event, error);
            }
            HttpResponse::InternalServerError().body(response_body)
        }
    }
}

fn organization_responses(organizations: Vec<OrganizationOutput>) -> Vec<OrganizationResponse> {
    organizations.into_iter().map(Into::into).collect()
}
