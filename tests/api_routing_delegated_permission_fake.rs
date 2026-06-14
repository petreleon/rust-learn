use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};
use rust_learn::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionOutput, DelegatedPermissionUseCase,
    GrantDelegatedPermissionCommand, ListDelegatedPermissionsQuery,
    RevokeDelegatedPermissionCommand,
};

struct RouteOnlyDelegatedPermissionUseCase;

pub fn delegated_permission_data() -> web::Data<Arc<dyn DelegatedPermissionUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyDelegatedPermissionUseCase) as Arc<dyn DelegatedPermissionUseCase>
    )
}

impl DelegatedPermissionUseCase for RouteOnlyDelegatedPermissionUseCase {
    fn grant_delegated_permission(
        &self,
        command: GrantDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            Ok(output(
                90,
                command.grantor_user_id,
                command.grantee_user_id,
                command.permission,
                command.scope_type,
                command.organization_id,
                command.course_id,
            ))
        }
        .boxed()
    }

    fn list_delegated_permissions(
        &self,
        query: ListDelegatedPermissionsQuery,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        async move {
            Ok(vec![output(
                90,
                query.actor_user_id,
                query.grantee_user_id.unwrap_or(20),
                query
                    .permission
                    .unwrap_or_else(|| "APPROVE_REWARD_AMOUNT".to_string()),
                query.scope_type.unwrap_or_else(|| "platform".to_string()),
                query.organization_id,
                query.course_id,
            )])
        }
        .boxed()
    }

    fn revoke_delegated_permission(
        &self,
        command: RevokeDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            let mut delegation = output(
                command.delegation_id,
                command.actor_user_id,
                20,
                "APPROVE_REWARD_AMOUNT".to_string(),
                "platform".to_string(),
                None,
                None,
            );
            delegation.revoke_reason = command.revoke_reason;
            delegation.revoked_at = Some(Utc::now());
            delegation.revoked_by_user_id = Some(command.actor_user_id);
            Ok(delegation)
        }
        .boxed()
    }
}

fn output(
    id: i64,
    grantor_user_id: i32,
    grantee_user_id: i32,
    permission: String,
    scope_type: String,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> DelegatedPermissionOutput {
    let now = Utc::now();
    DelegatedPermissionOutput {
        course_id,
        created_at: now,
        expires_at: None,
        grantee_user_id,
        grantor_user_id,
        id,
        organization_id,
        permission,
        reason: None,
        revoke_reason: None,
        revoked_at: None,
        revoked_by_user_id: None,
        scope_type,
        updated_at: now,
    }
}
