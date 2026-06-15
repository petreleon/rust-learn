use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionOutput, GrantDelegatedPermissionCommand, ListDelegatedPermissionsQuery,
    RevokeDelegatedPermissionCommand,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GrantDelegatedPermissionRequest {
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListDelegatedPermissionsParams {
    pub grantor_user_id: Option<i32>,
    pub grantee_user_id: Option<i32>,
    pub permission: Option<String>,
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RevokeDelegatedPermissionRequest {
    pub revoke_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DelegatedPermissionResponse {
    pub id: i64,
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
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

impl GrantDelegatedPermissionRequest {
    pub fn into_command(self, grantor_user_id: i32) -> GrantDelegatedPermissionCommand {
        GrantDelegatedPermissionCommand {
            course_id: self.course_id,
            expires_at: self.expires_at,
            grantee_user_id: self.grantee_user_id,
            grantor_user_id,
            organization_id: self.organization_id,
            permission: self.permission,
            reason: self.reason,
            scope_type: self.scope_type,
        }
    }
}

impl ListDelegatedPermissionsParams {
    pub fn into_query(self, actor_user_id: i32) -> ListDelegatedPermissionsQuery {
        ListDelegatedPermissionsQuery {
            active: self.active,
            actor_user_id,
            course_id: self.course_id,
            grantee_user_id: self.grantee_user_id,
            grantor_user_id: self.grantor_user_id,
            limit: self.limit,
            offset: self.offset,
            organization_id: self.organization_id,
            permission: self.permission,
            scope_type: self.scope_type,
        }
    }
}

impl RevokeDelegatedPermissionRequest {
    pub fn into_command(
        self,
        actor_user_id: i32,
        delegation_id: i64,
    ) -> RevokeDelegatedPermissionCommand {
        RevokeDelegatedPermissionCommand {
            actor_user_id,
            delegation_id,
            revoke_reason: self.revoke_reason,
        }
    }
}

impl From<DelegatedPermissionOutput> for DelegatedPermissionResponse {
    fn from(delegation: DelegatedPermissionOutput) -> Self {
        Self {
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
            scope_type: delegation.scope_type.as_str().to_string(),
            updated_at: delegation.updated_at,
        }
    }
}
