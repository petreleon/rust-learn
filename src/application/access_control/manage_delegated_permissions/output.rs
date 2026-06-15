use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedPermissionOutput {
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

pub(crate) struct DelegatedPermissionFact {
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

pub(crate) fn delegated_permission_output(
    fact: DelegatedPermissionFact,
) -> DelegatedPermissionOutput {
    DelegatedPermissionOutput {
        id: fact.id,
        grantor_user_id: fact.grantor_user_id,
        grantee_user_id: fact.grantee_user_id,
        permission: fact.permission,
        scope_type: fact.scope_type,
        organization_id: fact.organization_id,
        course_id: fact.course_id,
        reason: fact.reason,
        expires_at: fact.expires_at,
        revoked_at: fact.revoked_at,
        revoked_by_user_id: fact.revoked_by_user_id,
        revoke_reason: fact.revoke_reason,
        created_at: fact.created_at,
        updated_at: fact.updated_at,
    }
}
