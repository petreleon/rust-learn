use futures::future::BoxFuture;

use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationOutput,
    OrganizationUpdateCommand,
};

pub trait OrganizationManagementStore {
    fn list_organizations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>>;

    fn get_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn create_organization(
        &mut self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn update_organization(
        &mut self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>>;

    fn delete_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationManagementError>>;
}
