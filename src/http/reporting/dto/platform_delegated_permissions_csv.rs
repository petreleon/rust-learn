use crate::application::reporting::platform_csv_exports::PlatformDelegatedPermissionExportRowOutput;
use crate::http::reporting::dto::csv::{csv_optional, csv_value};

pub fn platform_delegated_permissions_csv(
    rows: &[PlatformDelegatedPermissionExportRowOutput],
) -> String {
    let mut csv = String::from(
        "delegated_permission_id,grantor_user_id,grantee_user_id,permission,scope_type,organization_id,course_id,state,reason,expires_at,revoked_at,revoked_by_user_id,revoke_reason,created_at,updated_at\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.delegated_permission_id,
            row.grantor_user_id,
            row.grantee_user_id,
            csv_value(&row.permission),
            csv_value(row.scope_type.as_str()),
            csv_optional(row.organization_id),
            csv_optional(row.course_id),
            csv_value(row.state.as_str()),
            csv_value(&row.reason),
            csv_optional(row.expires_at.as_ref()),
            csv_optional(row.revoked_at.as_ref()),
            csv_optional(row.revoked_by_user_id),
            csv_value(&row.revoke_reason),
            row.created_at,
            row.updated_at
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::platform_delegated_permissions_csv;
    use crate::application::reporting::platform_csv_exports::{
        PlatformDelegatedPermissionExportRowOutput, PlatformDelegatedPermissionExportState,
    };
    use crate::domain::access_control::delegation::DelegatedScopeType;

    #[test]
    fn keeps_legacy_delegated_permissions_csv_shape() {
        let now = Utc::now();
        let csv =
            platform_delegated_permissions_csv(&[PlatformDelegatedPermissionExportRowOutput {
                delegated_permission_id: 1,
                grantor_user_id: 2,
                grantee_user_id: 3,
                permission: "APPROVE_REWARD_AMOUNT".to_string(),
                scope_type: DelegatedScopeType::Platform,
                organization_id: None,
                course_id: None,
                state: PlatformDelegatedPermissionExportState::Active,
                reason: "temporary, reviewer".to_string(),
                expires_at: None,
                revoked_at: None,
                revoked_by_user_id: None,
                revoke_reason: String::new(),
                created_at: now,
                updated_at: now,
            }]);

        assert!(csv.starts_with("delegated_permission_id,grantor_user_id"));
        assert!(csv.contains("APPROVE_REWARD_AMOUNT"));
        assert!(csv.contains("\"temporary, reviewer\""));
    }
}
