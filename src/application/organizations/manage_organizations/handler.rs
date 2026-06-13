use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementStore,
    OrganizationOutput, OrganizationUpdateCommand,
};

pub async fn list_organizations(
    store: &mut impl OrganizationManagementStore,
) -> Result<Vec<OrganizationOutput>, OrganizationManagementError> {
    store.list_organizations().await
}

pub async fn get_organization(
    store: &mut impl OrganizationManagementStore,
    organization_id: i32,
) -> Result<OrganizationOutput, OrganizationManagementError> {
    store.get_organization(organization_id).await
}

pub async fn create_organization(
    store: &mut impl OrganizationManagementStore,
    command: OrganizationCreateCommand,
) -> Result<OrganizationOutput, OrganizationManagementError> {
    store.create_organization(command).await
}

pub async fn update_organization(
    store: &mut impl OrganizationManagementStore,
    command: OrganizationUpdateCommand,
) -> Result<OrganizationOutput, OrganizationManagementError> {
    store.update_organization(command).await
}

pub async fn delete_organization(
    store: &mut impl OrganizationManagementStore,
    organization_id: i32,
) -> Result<(), OrganizationManagementError> {
    if store.delete_organization(organization_id).await? {
        Ok(())
    } else {
        Err(OrganizationManagementError::NotFound)
    }
}

#[cfg(test)]
mod tests;
