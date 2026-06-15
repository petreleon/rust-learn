use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalStore,
};

const MANAGE_ORG_MEMBERS: &str = "MANAGE_ORG_MEMBERS";

pub async fn remove_organization_member(
    store: &mut impl OrganizationMemberRemovalStore,
    command: OrganizationMemberRemovalCommand,
) -> Result<(), OrganizationMemberRemovalError> {
    if !store
        .can(
            AccessActor::user(command.actor_user_id),
            AccessAction::permission(MANAGE_ORG_MEMBERS),
            AccessScope::organization(command.organization_id),
        )
        .await?
    {
        return Err(OrganizationMemberRemovalError::PermissionDenied);
    }

    store.remove_member(command).await
}

#[cfg(test)]
mod tests;
