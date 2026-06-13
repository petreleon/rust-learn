use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformDelegatedPermissionExportRowOutput,
};
use crate::db::schema::delegated_permissions;
use crate::infra::postgres::reporting::platform_csv_export_store::map_diesel_error;
use crate::models::delegated_permission::DelegatedPermission;

pub(super) async fn load_delegated_permission_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError> {
    let now = chrono::Utc::now();
    delegated_permissions::table
        .order(delegated_permissions::created_at.desc())
        .limit(1000)
        .load::<DelegatedPermission>(conn)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|delegation| map_delegated_permission(delegation, now))
                .collect()
        })
        .map_err(map_diesel_error)
}

fn map_delegated_permission(
    delegation: DelegatedPermission,
    now: chrono::DateTime<chrono::Utc>,
) -> PlatformDelegatedPermissionExportRowOutput {
    let state = if delegation.revoked_at.is_some() {
        "revoked"
    } else if delegation
        .expires_at
        .map(|expires_at| expires_at <= now)
        .unwrap_or(false)
    {
        "expired"
    } else {
        "active"
    };

    PlatformDelegatedPermissionExportRowOutput {
        delegated_permission_id: delegation.id,
        grantor_user_id: delegation.grantor_user_id,
        grantee_user_id: delegation.grantee_user_id,
        permission: delegation.permission,
        scope_type: delegation.scope_type,
        organization_id: delegation.organization_id,
        course_id: delegation.course_id,
        state: state.to_string(),
        reason: delegation.reason.unwrap_or_default(),
        expires_at: delegation.expires_at,
        revoked_at: delegation.revoked_at,
        revoked_by_user_id: delegation.revoked_by_user_id,
        revoke_reason: delegation.revoke_reason.unwrap_or_default(),
        created_at: delegation.created_at,
        updated_at: delegation.updated_at,
    }
}
