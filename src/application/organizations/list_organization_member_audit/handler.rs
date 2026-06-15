use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn list_organization_member_audit(
    store: &mut impl OrganizationMemberAuditStore,
    query: OrganizationMemberAuditQuery,
) -> Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError> {
    ensure_can_view_member_audit(store, &query).await?;
    store.list_member_audit_events(query).await
}

async fn ensure_can_view_member_audit(
    store: &mut impl OrganizationMemberAuditStore,
    query: &OrganizationMemberAuditQuery,
) -> Result<(), OrganizationMemberAuditError> {
    if store
        .can(
            AccessActor::user(query.actor_user_id),
            AccessAction::permission(Permissions::VIEW_ORGANIZATION),
            AccessScope::organization(query.organization_id),
        )
        .await?
    {
        Ok(())
    } else {
        Err(OrganizationMemberAuditError::PermissionDenied(
            Permissions::VIEW_ORGANIZATION.into(),
        ))
    }
}

#[cfg(test)]
mod tests;
