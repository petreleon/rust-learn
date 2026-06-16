use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::identity::list_platform_role_assignment_audit::{
    PlatformRoleAssignmentAuditError, PlatformRoleAssignmentAuditEventOutput,
    PlatformRoleAssignmentAuditQuery, PlatformRoleAssignmentAuditStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn list_platform_role_assignment_audit(
    store: &mut impl PlatformRoleAssignmentAuditStore,
    query: PlatformRoleAssignmentAuditQuery,
) -> Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError> {
    ensure_can_view_role_assignments(store, query.actor_user_id).await?;
    if !store.target_user_exists(query.target_user_id).await? {
        return Err(PlatformRoleAssignmentAuditError::UserNotFound);
    }
    store.list_assignment_audit(query.target_user_id).await
}

async fn ensure_can_view_role_assignments(
    store: &mut impl PlatformRoleAssignmentAuditStore,
    actor_user_id: i32,
) -> Result<(), PlatformRoleAssignmentAuditError> {
    let permission = Permissions::VIEW_ROLE_ASSIGNMENTS.to_string();
    let allowed = store
        .can(
            AccessActor::user(actor_user_id),
            AccessAction::permission(permission.clone()),
            AccessScope::platform(),
        )
        .await?;
    if allowed {
        Ok(())
    } else {
        Err(PlatformRoleAssignmentAuditError::PermissionDenied(
            permission,
        ))
    }
}

#[cfg(test)]
mod tests;
