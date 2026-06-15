use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput,
};
use crate::domain::organizations::member_audit::OrganizationMemberAuditEventType;
use crate::models::organization_member_audit_event::OrganizationMemberAuditEvent;

pub fn organization_member_audit_output_from_model(
    event: OrganizationMemberAuditEvent,
) -> Result<OrganizationMemberAuditEventOutput, OrganizationMemberAuditError> {
    let event_type = OrganizationMemberAuditEventType::parse(&event.event_type)
        .map_err(|error| OrganizationMemberAuditError::Database(error.to_string()))?;
    Ok(OrganizationMemberAuditEventOutput {
        id: event.id,
        organization_id: event.organization_id,
        actor_user_id: event.actor_user_id,
        target_user_id: event.target_user_id,
        event_type,
        role_name: event.role_name,
        reason: event.reason,
        created_at: event.created_at,
    })
}

pub fn map_member_audit_error(error: diesel::result::Error) -> OrganizationMemberAuditError {
    OrganizationMemberAuditError::Database(error.to_string())
}
