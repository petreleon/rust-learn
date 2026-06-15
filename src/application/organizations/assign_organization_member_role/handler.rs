use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentStore,
};

const ASSIGN_ROLES_TO_ORG_USERS: &str = "ASSIGN_ROLES_TO_ORG_USERS";

pub async fn assign_organization_member_role(
    store: &mut impl OrganizationMemberRoleAssignmentStore,
    command: OrganizationMemberRoleAssignmentCommand,
) -> Result<OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentError> {
    if !store
        .can(
            AccessActor::user(command.actor_user_id),
            AccessAction::permission(ASSIGN_ROLES_TO_ORG_USERS),
            AccessScope::organization(command.organization_id),
        )
        .await?
    {
        return Err(OrganizationMemberRoleAssignmentError::PermissionDenied);
    }

    let actor_level = store
        .actor_min_level(command.actor_user_id, command.organization_id)
        .await?
        .ok_or(OrganizationMemberRoleAssignmentError::NotFound)?;
    let target_level = store
        .target_min_level(command.target_user_id, command.organization_id)
        .await?;
    let role_id = store
        .role_id_by_name(&command.role_name)
        .await?
        .ok_or(OrganizationMemberRoleAssignmentError::NotFound)?;
    let role_level = store
        .role_hierarchy_level(role_id)
        .await?
        .ok_or(OrganizationMemberRoleAssignmentError::NotFound)?;

    if actor_level >= role_level || target_level.is_some_and(|level| actor_level >= level) {
        return Err(OrganizationMemberRoleAssignmentError::HierarchyViolation);
    }

    store
        .assign_role(command.target_user_id, command.organization_id, role_id)
        .await?;
    store.record_role_assignment(&command).await.ok();

    Ok(OrganizationMemberRoleAssignmentOutput {
        organization_id: command.organization_id,
        target_user_id: command.target_user_id,
        role_name: command.role_name,
    })
}

#[cfg(test)]
mod tests;
