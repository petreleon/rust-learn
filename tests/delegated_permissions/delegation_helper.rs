use crate::support::*;

pub(crate) struct GrantDelegatedPermissionRequest {
    pub(crate) grantee_user_id: i32,
    pub(crate) permission: String,
    pub(crate) scope_type: String,
    pub(crate) organization_id: Option<i32>,
    pub(crate) course_id: Option<i32>,
    pub(crate) reason: Option<String>,
    pub(crate) expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub(crate) async fn grant_delegated_permission(
    _conn: &mut AsyncPgConnection,
    grantor_user_id: i32,
    request: GrantDelegatedPermissionRequest,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    let pool = establish_connection();
    PostgresDelegatedPermissionUseCase::new(pool)
        .grant_delegated_permission(GrantDelegatedPermissionCommand {
            course_id: request.course_id,
            expires_at: request.expires_at,
            grantee_user_id: request.grantee_user_id,
            grantor_user_id,
            organization_id: request.organization_id,
            permission: request.permission,
            reason: request.reason,
            scope_type: request.scope_type,
        })
        .await
}

pub(crate) async fn revoke_delegated_permission(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    delegation_id: i64,
    revoke_reason: Option<String>,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    let pool = establish_connection();
    PostgresDelegatedPermissionUseCase::new(pool)
        .revoke_delegated_permission(RevokeDelegatedPermissionCommand {
            actor_user_id,
            delegation_id,
            revoke_reason,
        })
        .await
}
