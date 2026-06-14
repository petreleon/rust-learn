struct GrantDelegatedPermissionRequest {
    grantee_user_id: i32,
    permission: String,
    scope_type: String,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    reason: Option<String>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn grant_delegated_permission(
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

async fn revoke_delegated_permission(
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
