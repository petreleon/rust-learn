use chrono::Utc;
use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};

use super::list_organization_member_audit;
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditStore,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::organizations::member_audit::OrganizationMemberAuditEventType;

#[test]
fn lists_events_after_permission_check() {
    let mut store = FakeOrganizationMemberAuditStore::allowed();
    let query = OrganizationMemberAuditQuery {
        actor_user_id: 7,
        organization_id: 9,
        target_user_id: 11,
    };

    let events = block_on(list_organization_member_audit(&mut store, query))
        .expect("member audit should load");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].target_user_id, 11);
    assert!(store.checked_permission);
    assert!(store.listed_events);
}

#[test]
fn rejects_without_listing_events() {
    let mut store = FakeOrganizationMemberAuditStore::denied();
    let query = OrganizationMemberAuditQuery {
        actor_user_id: 7,
        organization_id: 9,
        target_user_id: 11,
    };

    let error = block_on(list_organization_member_audit(&mut store, query))
        .expect_err("member audit should be permission gated");

    assert_eq!(
        error,
        OrganizationMemberAuditError::PermissionDenied(Permissions::VIEW_ORGANIZATION.into())
    );
    assert!(store.checked_permission);
    assert!(!store.listed_events);
}

struct FakeOrganizationMemberAuditStore {
    can_view: bool,
    checked_permission: bool,
    listed_events: bool,
}

impl FakeOrganizationMemberAuditStore {
    fn allowed() -> Self {
        Self {
            can_view: true,
            checked_permission: false,
            listed_events: false,
        }
    }

    fn denied() -> Self {
        Self {
            can_view: false,
            checked_permission: false,
            listed_events: false,
        }
    }
}

impl AccessDecisionStore for FakeOrganizationMemberAuditStore {
    type Error = OrganizationMemberAuditError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberAuditError>> {
        assert_eq!(
            action.permission_name(),
            Permissions::VIEW_ORGANIZATION.to_string()
        );
        assert!(matches!(scope, AccessScope::Organization(_)));
        self.checked_permission = true;
        ready(Ok(self.can_view)).boxed()
    }
}

impl OrganizationMemberAuditStore for FakeOrganizationMemberAuditStore {
    fn list_member_audit_events(
        &mut self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>
    {
        self.listed_events = true;
        ready(Ok(vec![OrganizationMemberAuditEventOutput {
            id: 1,
            organization_id: query.organization_id,
            actor_user_id: Some(query.actor_user_id),
            target_user_id: query.target_user_id,
            event_type: OrganizationMemberAuditEventType::RoleAssigned,
            role_name: Some("STUDENT".to_string()),
            reason: Some("test fixture".to_string()),
            created_at: Utc::now(),
        }]))
        .boxed()
    }
}
