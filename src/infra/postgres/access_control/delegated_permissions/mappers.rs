use crate::application::access_control::manage_delegated_permissions::{
    delegated_permission_output, DelegatedPermissionError, DelegatedPermissionFact,
    DelegatedPermissionOutput,
};
use crate::models::delegated_permission::DelegatedPermission;

pub(super) fn delegated_permission_output_from_record(
    delegation: DelegatedPermission,
) -> DelegatedPermissionOutput {
    delegated_permission_output(DelegatedPermissionFact {
        course_id: delegation.course_id,
        created_at: delegation.created_at,
        expires_at: delegation.expires_at,
        grantee_user_id: delegation.grantee_user_id,
        grantor_user_id: delegation.grantor_user_id,
        id: delegation.id,
        organization_id: delegation.organization_id,
        permission: delegation.permission,
        reason: delegation.reason,
        revoke_reason: delegation.revoke_reason,
        revoked_at: delegation.revoked_at,
        revoked_by_user_id: delegation.revoked_by_user_id,
        scope_type: delegation.scope_type,
        updated_at: delegation.updated_at,
    })
}

pub(super) fn map_error(error: diesel::result::Error) -> DelegatedPermissionError {
    match error {
        diesel::result::Error::NotFound => DelegatedPermissionError::NotFound,
        other => DelegatedPermissionError::Database(other.to_string()),
    }
}
