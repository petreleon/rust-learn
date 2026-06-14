use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteCommand, OrganizationMemberInviteError, OrganizationMemberInviteOutput,
    OrganizationMemberInviteStore,
};

pub async fn invite_organization_member(
    store: &mut impl OrganizationMemberInviteStore,
    command: OrganizationMemberInviteCommand,
) -> Result<OrganizationMemberInviteOutput, OrganizationMemberInviteError> {
    if !store
        .can_invite_member(command.actor_user_id, command.organization_id)
        .await?
    {
        return Err(OrganizationMemberInviteError::PermissionDenied);
    }

    let role_name = command.requested_role_name();
    let target = store.find_user_by_email(command.lookup_email()).await?;
    store
        .assign_member_role(
            command.actor_user_id,
            target.user_id,
            command.organization_id,
            role_name.clone(),
        )
        .await?;

    Ok(OrganizationMemberInviteOutput {
        user_id: target.user_id,
        name: target.name,
        email: target.email,
        role: role_name,
    })
}

#[cfg(test)]
mod tests;
