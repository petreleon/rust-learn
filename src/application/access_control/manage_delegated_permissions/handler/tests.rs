use super::*;
use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionCreate, DelegatedPermissionFilter, DelegatedPermissionOutput,
    DelegatedPermissionStore, GrantDelegatedPermissionCommand, ListDelegatedPermissionsQuery,
};
use crate::domain::access_control::delegation::DelegatedScopeType;
use chrono::Utc;
use futures::future::{BoxFuture, FutureExt};

#[derive(Default)]
struct FakeStore {
    active: Option<DelegatedPermissionOutput>,
    can_delegate: bool,
    created: Option<DelegatedPermissionCreate>,
    listed: Option<DelegatedPermissionFilter>,
}

impl DelegatedPermissionStore for FakeStore {
    fn can_delegate_reward_permissions(
        &mut self,
        _user_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { Ok(self.can_delegate) }.boxed()
    }

    fn organization_exists(
        &mut self,
        _: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { Ok(true) }.boxed()
    }

    fn course_exists(&mut self, _: i32) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { Ok(true) }.boxed()
    }

    fn find_active_delegated_permission(
        &mut self,
        _: i32,
        _: String,
        _: DelegatedScopeType,
        _: Option<i32>,
        _: Option<i32>,
    ) -> BoxFuture<'_, Result<Option<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        async move { Ok(self.active.clone()) }.boxed()
    }
    fn create_delegated_permission(
        &mut self,
        delegation: DelegatedPermissionCreate,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        self.created = Some(delegation.clone());
        async move {
            Ok(output(
                &delegation.permission,
                delegation.scope_type,
                delegation.organization_id,
                delegation.course_id,
            ))
        }
        .boxed()
    }

    fn list_delegated_permissions(
        &mut self,
        filter: DelegatedPermissionFilter,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        self.listed = Some(filter);
        async move { Ok(Vec::new()) }.boxed()
    }
    fn revoke_delegated_permission(
        &mut self,
        _: i64,
        _: i32,
        _: Option<String>,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            Ok(output(
                "APPROVE_REWARD_AMOUNT",
                DelegatedScopeType::Platform,
                None,
                None,
            ))
        }
        .boxed()
    }
}
#[tokio::test]
async fn grant_normalizes_and_creates_delegation() {
    let mut store = FakeStore {
        can_delegate: true,
        ..Default::default()
    };

    let granted = grant_delegated_permission(&mut store, platform_command())
        .await
        .unwrap();

    let created = store.created.expect("delegation should be created");
    assert_eq!(created.permission, "APPROVE_REWARD_AMOUNT");
    assert_eq!(created.scope_type, DelegatedScopeType::Platform);
    assert_eq!(granted.grantee_user_id, 20);
}

#[tokio::test]
async fn grant_requires_delegate_permission_before_creating() {
    let mut store = FakeStore::default();

    let error = grant_delegated_permission(&mut store, platform_command())
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        DelegatedPermissionError::PermissionDenied(permission)
            if permission == "DELEGATE_REWARD_APPROVAL"
    ));
    assert!(store.created.is_none());
}

#[tokio::test]
async fn list_normalizes_filters_before_store_query() {
    let mut store = FakeStore {
        can_delegate: true,
        ..Default::default()
    };

    list_delegated_permissions(
        &mut store,
        ListDelegatedPermissionsQuery {
            actor_user_id: 10,
            permission: Some(" VIEW_REWARD_AUDIT ".to_string()),
            scope_type: Some("PLATFORM".to_string()),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let filter = store.listed.expect("filter should be passed to store");
    assert_eq!(filter.permission.as_deref(), Some("VIEW_REWARD_AUDIT"));
    assert_eq!(filter.scope_type, Some(DelegatedScopeType::Platform));
}

fn platform_command() -> GrantDelegatedPermissionCommand {
    GrantDelegatedPermissionCommand {
        course_id: None,
        expires_at: None,
        grantee_user_id: 20,
        grantor_user_id: 10,
        organization_id: None,
        permission: "APPROVE_REWARD_AMOUNT".to_string(),
        reason: None,
        scope_type: " PLATFORM ".to_string(),
    }
}

fn output(
    permission: &str,
    scope_type: DelegatedScopeType,
    organization_id: Option<i32>,
    course_id: Option<i32>,
) -> DelegatedPermissionOutput {
    let now = Utc::now();
    DelegatedPermissionOutput {
        course_id,
        created_at: now,
        expires_at: None,
        grantee_user_id: 20,
        grantor_user_id: 10,
        id: 1,
        organization_id,
        permission: permission.to_string(),
        reason: None,
        revoke_reason: None,
        revoked_at: None,
        revoked_by_user_id: None,
        scope_type,
        updated_at: now,
    }
}
