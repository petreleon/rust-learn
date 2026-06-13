use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantDelegatedPermissionCommand {
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeDelegatedPermissionCommand {
    pub actor_user_id: i32,
    pub delegation_id: i64,
    pub revoke_reason: Option<String>,
}
