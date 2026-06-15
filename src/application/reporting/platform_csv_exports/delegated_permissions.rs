use chrono::{DateTime, Utc};

use crate::application::reporting::platform_csv_exports::{
    PlatformDelegatedPermissionExportRowOutput, PlatformDelegatedPermissionExportState,
};
use crate::domain::access_control::delegation::DelegatedScopeType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlatformDelegatedPermissionExportFact {
    pub delegated_permission_id: i64,
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: DelegatedScopeType,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<i32>,
    pub revoke_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) fn platform_delegated_permission_export_row(
    fact: PlatformDelegatedPermissionExportFact,
) -> PlatformDelegatedPermissionExportRowOutput {
    platform_delegated_permission_export_row_at(fact, Utc::now())
}

fn platform_delegated_permission_export_row_at(
    fact: PlatformDelegatedPermissionExportFact,
    now: DateTime<Utc>,
) -> PlatformDelegatedPermissionExportRowOutput {
    let state = if fact.revoked_at.is_some() {
        PlatformDelegatedPermissionExportState::Revoked
    } else if fact
        .expires_at
        .map(|expires_at| expires_at <= now)
        .unwrap_or(false)
    {
        PlatformDelegatedPermissionExportState::Expired
    } else {
        PlatformDelegatedPermissionExportState::Active
    };

    PlatformDelegatedPermissionExportRowOutput {
        delegated_permission_id: fact.delegated_permission_id,
        grantor_user_id: fact.grantor_user_id,
        grantee_user_id: fact.grantee_user_id,
        permission: fact.permission,
        scope_type: fact.scope_type,
        organization_id: fact.organization_id,
        course_id: fact.course_id,
        state,
        reason: fact.reason.unwrap_or_default(),
        expires_at: fact.expires_at,
        revoked_at: fact.revoked_at,
        revoked_by_user_id: fact.revoked_by_user_id,
        revoke_reason: fact.revoke_reason.unwrap_or_default(),
        created_at: fact.created_at,
        updated_at: fact.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn revoked_delegation_wins_over_expiry() {
        let now = Utc::now();

        let row = platform_delegated_permission_export_row_at(
            delegated_permission_fact(
                Some(now - Duration::days(1)),
                Some(now - Duration::days(2)),
                Some("grant reason".to_string()),
                Some("revoke reason".to_string()),
                now,
            ),
            now,
        );

        assert_eq!(row.state, PlatformDelegatedPermissionExportState::Revoked);
        assert_eq!(row.reason, "grant reason");
        assert_eq!(row.revoke_reason, "revoke reason");
    }

    #[test]
    fn expired_delegation_is_marked_expired() {
        let now = Utc::now();

        let row = platform_delegated_permission_export_row_at(
            delegated_permission_fact(Some(now - Duration::seconds(1)), None, None, None, now),
            now,
        );

        assert_eq!(row.state, PlatformDelegatedPermissionExportState::Expired);
        assert_eq!(row.reason, "");
        assert_eq!(row.revoke_reason, "");
    }

    #[test]
    fn non_expired_delegation_is_active() {
        let now = Utc::now();

        let row = platform_delegated_permission_export_row_at(
            delegated_permission_fact(Some(now + Duration::seconds(1)), None, None, None, now),
            now,
        );

        assert_eq!(row.state, PlatformDelegatedPermissionExportState::Active);
    }

    fn delegated_permission_fact(
        expires_at: Option<DateTime<Utc>>,
        revoked_at: Option<DateTime<Utc>>,
        reason: Option<String>,
        revoke_reason: Option<String>,
        now: DateTime<Utc>,
    ) -> PlatformDelegatedPermissionExportFact {
        PlatformDelegatedPermissionExportFact {
            delegated_permission_id: 1,
            grantor_user_id: 2,
            grantee_user_id: 3,
            permission: "reports:read".to_string(),
            scope_type: DelegatedScopeType::Platform,
            organization_id: None,
            course_id: None,
            reason,
            expires_at,
            revoked_at,
            revoked_by_user_id: Some(4),
            revoke_reason,
            created_at: now,
            updated_at: now,
        }
    }
}
