use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalStore,
};

pub async fn remove_organization_member(
    store: &mut impl OrganizationMemberRemovalStore,
    command: OrganizationMemberRemovalCommand,
) -> Result<(), OrganizationMemberRemovalError> {
    if !store
        .can_remove_member(command.actor_user_id, command.organization_id)
        .await?
    {
        return Err(OrganizationMemberRemovalError::PermissionDenied);
    }

    store.remove_member(command).await
}

#[cfg(test)]
mod tests;
