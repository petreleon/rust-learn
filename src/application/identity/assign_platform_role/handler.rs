use super::{
    AssignPlatformRoleCommand, AssignPlatformRoleError, AssignPlatformRoleOutcome,
    PlatformRoleAssignmentStore,
};

pub async fn assign_platform_role(
    store: &mut impl PlatformRoleAssignmentStore,
    command: AssignPlatformRoleCommand,
) -> Result<AssignPlatformRoleOutcome, AssignPlatformRoleError> {
    store.assign_role(command.clone()).await?;
    Ok(AssignPlatformRoleOutcome {
        target_user_id: command.target_user_id,
        role_name: command.role_name,
    })
}

#[cfg(test)]
mod tests;
