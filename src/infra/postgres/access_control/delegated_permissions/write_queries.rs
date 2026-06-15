use diesel_async::AsyncPgConnection;

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionCreate, DelegatedPermissionError, DelegatedPermissionOutput,
};
use crate::infra::postgres::access_control::delegated_permissions::mappers::{
    delegated_permission_output_from_record, map_error,
};
use crate::infra::postgres::access_control::delegated_permissions::records;
use crate::models::delegated_permission::NewDelegatedPermission;

pub(super) async fn create_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation: DelegatedPermissionCreate,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    let new_delegation = NewDelegatedPermission {
        course_id: delegation.course_id,
        expires_at: delegation.expires_at,
        grantee_user_id: delegation.grantee_user_id,
        grantor_user_id: delegation.grantor_user_id,
        organization_id: delegation.organization_id,
        permission: delegation.permission,
        reason: delegation.reason,
        scope_type: delegation.scope_type,
    };

    records::create_delegated_permission(conn, new_delegation)
        .await
        .map(delegated_permission_output_from_record)
        .map_err(map_error)
}

pub(super) async fn revoke_delegated_permission(
    conn: &mut AsyncPgConnection,
    delegation_id: i64,
    revoked_by_user_id: i32,
    revoke_reason: Option<String>,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    records::revoke_delegated_permission(conn, delegation_id, revoked_by_user_id, revoke_reason)
        .await
        .map(delegated_permission_output_from_record)
        .map_err(map_error)
}
