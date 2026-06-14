use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditStore,
};

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
        .can_view_member_audit(query.actor_user_id, query.organization_id)
        .await?
    {
        Ok(())
    } else {
        Err(OrganizationMemberAuditError::PermissionDenied(
            "VIEW_ORGANIZATION".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests;
