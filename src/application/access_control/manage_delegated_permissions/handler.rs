use chrono::Utc;

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionCreate, DelegatedPermissionError, DelegatedPermissionFilter,
    DelegatedPermissionOutput, DelegatedPermissionStore, GrantDelegatedPermissionCommand,
    ListDelegatedPermissionsQuery, RevokeDelegatedPermissionCommand,
};
use crate::domain::access_control::delegation::{
    normalize_filter_permission, normalize_permission, normalize_scope_ids, DelegatedScopeType,
};

pub async fn grant_delegated_permission(
    store: &mut impl DelegatedPermissionStore,
    command: GrantDelegatedPermissionCommand,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    ensure_delegate_permission(store, command.grantor_user_id).await?;
    let permission = normalize_permission(&command.permission)?;
    let scope_type = DelegatedScopeType::normalize(&command.scope_type)?;
    let scope = normalize_scope_ids(
        &permission,
        scope_type.as_str(),
        command.organization_id,
        command.course_id,
    )?;
    ensure_scope_exists(store, scope.organization_id, scope.course_id).await?;

    if let Some(expires_at) = command.expires_at {
        if expires_at <= Utc::now() {
            return Err(DelegatedPermissionError::InvalidInput(
                "delegated permission expiration must be in the future".to_string(),
            ));
        }
    }

    if let Some(existing) = store
        .find_active_delegated_permission(
            command.grantee_user_id,
            permission.clone(),
            scope_type,
            scope.organization_id,
            scope.course_id,
        )
        .await?
    {
        return Ok(existing);
    }

    store
        .create_delegated_permission(DelegatedPermissionCreate {
            course_id: scope.course_id,
            expires_at: command.expires_at,
            grantee_user_id: command.grantee_user_id,
            grantor_user_id: command.grantor_user_id,
            organization_id: scope.organization_id,
            permission,
            reason: command.reason,
            scope_type,
        })
        .await
}

pub async fn list_delegated_permissions(
    store: &mut impl DelegatedPermissionStore,
    query: ListDelegatedPermissionsQuery,
) -> Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError> {
    ensure_delegate_permission(store, query.actor_user_id).await?;
    store
        .list_delegated_permissions(DelegatedPermissionFilter {
            active: query.active,
            course_id: query.course_id,
            grantee_user_id: query.grantee_user_id,
            grantor_user_id: query.grantor_user_id,
            limit: query.limit,
            offset: query.offset,
            organization_id: query.organization_id,
            permission: normalize_filter_permission(query.permission)?,
            scope_type: query
                .scope_type
                .as_deref()
                .map(DelegatedScopeType::normalize)
                .transpose()?,
        })
        .await
}

pub async fn revoke_delegated_permission(
    store: &mut impl DelegatedPermissionStore,
    command: RevokeDelegatedPermissionCommand,
) -> Result<DelegatedPermissionOutput, DelegatedPermissionError> {
    ensure_delegate_permission(store, command.actor_user_id).await?;
    store
        .revoke_delegated_permission(
            command.delegation_id,
            command.actor_user_id,
            command.revoke_reason,
        )
        .await
}

async fn ensure_delegate_permission(
    store: &mut impl DelegatedPermissionStore,
    user_id: i32,
) -> Result<(), DelegatedPermissionError> {
    let permission = "DELEGATE_REWARD_APPROVAL";
    if store.can_delegate_reward_permissions(user_id).await? {
        Ok(())
    } else {
        Err(DelegatedPermissionError::PermissionDenied(
            permission.to_string(),
        ))
    }
}

async fn ensure_scope_exists(
    store: &mut impl DelegatedPermissionStore,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> Result<(), DelegatedPermissionError> {
    if let Some(organization_id) = organization_id {
        if !store.organization_exists(organization_id).await? {
            return Err(DelegatedPermissionError::InvalidInput(
                "organization scope does not exist".to_string(),
            ));
        }
    }
    if let Some(course_id) = course_id {
        if !store.course_exists(course_id).await? {
            return Err(DelegatedPermissionError::InvalidInput(
                "course scope does not exist".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
