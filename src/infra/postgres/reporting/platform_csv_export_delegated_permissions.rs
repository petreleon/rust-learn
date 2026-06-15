use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    platform_delegated_permission_export_row, PlatformCsvExportError,
    PlatformDelegatedPermissionExportFact, PlatformDelegatedPermissionExportRowOutput,
};
use crate::db::schema::delegated_permissions;
use crate::domain::access_control::delegation::DelegatedScopeType;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::models::delegated_permission::DelegatedPermission;

pub(super) async fn load_delegated_permission_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformDelegatedPermissionExportRowOutput>, PlatformCsvExportError> {
    delegated_permissions::table
        .order(delegated_permissions::created_at.desc())
        .limit(1000)
        .load::<DelegatedPermission>(conn)
        .await
        .map_err(map_diesel_error)?
        .into_iter()
        .map(delegated_permission_fact)
        .map(|fact| fact.map(platform_delegated_permission_export_row))
        .collect()
}

fn delegated_permission_fact(
    delegation: DelegatedPermission,
) -> Result<PlatformDelegatedPermissionExportFact, PlatformCsvExportError> {
    let scope_type = DelegatedScopeType::parse(&delegation.scope_type)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;

    Ok(PlatformDelegatedPermissionExportFact {
        delegated_permission_id: delegation.id,
        grantor_user_id: delegation.grantor_user_id,
        grantee_user_id: delegation.grantee_user_id,
        permission: delegation.permission,
        scope_type,
        organization_id: delegation.organization_id,
        course_id: delegation.course_id,
        reason: delegation.reason,
        expires_at: delegation.expires_at,
        revoked_at: delegation.revoked_at,
        revoked_by_user_id: delegation.revoked_by_user_id,
        revoke_reason: delegation.revoke_reason,
        created_at: delegation.created_at,
        updated_at: delegation.updated_at,
    })
}
