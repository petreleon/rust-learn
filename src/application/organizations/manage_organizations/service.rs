use futures::future::BoxFuture;

use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationOutput,
    OrganizationUpdateCommand,
};

pub trait OrganizationManagementUseCase: Send + Sync {
    fn list_organizations(
        &self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>>;

    fn get_organization(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn create_organization(
        &self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn update_organization(
        &self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn delete_organization(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationManagementError>>;
}
