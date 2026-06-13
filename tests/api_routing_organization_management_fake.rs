use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementUseCase,
    OrganizationOutput, OrganizationUpdateCommand,
};

struct RouteOnlyOrganizationManagementUseCase;

pub fn organization_management_data() -> web::Data<Arc<dyn OrganizationManagementUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationManagementUseCase) as Arc<dyn OrganizationManagementUseCase>
    )
}

impl OrganizationManagementUseCase for RouteOnlyOrganizationManagementUseCase {
    fn list_organizations(
        &self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>> {
        ready(Ok(Vec::new())).boxed()
    }

    fn get_organization(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        ready(Ok(output(organization_id))).boxed()
    }

    fn create_organization(
        &self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        ready(Ok(OrganizationOutput {
            id: 56,
            name: command.name,
            website_link: command.website_link,
            profile_url: command.profile_url,
        }))
        .boxed()
    }

    fn update_organization(
        &self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        ready(Ok(OrganizationOutput {
            id: command.organization_id,
            name: command.name.unwrap_or_else(|| "Updated".to_string()),
            website_link: command.website_link,
            profile_url: command.profile_url,
        }))
        .boxed()
    }

    fn delete_organization(
        &self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationManagementError>> {
        ready(Ok(())).boxed()
    }
}

fn output(id: i32) -> OrganizationOutput {
    OrganizationOutput {
        id,
        name: "Route smoke".to_string(),
        website_link: None,
        profile_url: None,
    }
}
