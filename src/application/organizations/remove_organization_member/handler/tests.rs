use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};

use super::{remove_organization_member, MANAGE_ORG_MEMBERS};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalStore,
};

#[test]
fn removes_member_after_permission_check() {
    let mut store = FakeOrganizationMemberRemovalStore::allowed();
    let command = command();

    block_on(remove_organization_member(&mut store, command))
        .expect("member removal should succeed");

    assert_eq!(store.permission_check, Some((7, 9)));
    assert_eq!(store.removed_member, Some((9, 11)));
}

#[test]
fn rejects_without_removing_member() {
    let mut store = FakeOrganizationMemberRemovalStore::denied();

    let error = block_on(remove_organization_member(&mut store, command()))
        .expect_err("member removal should be permission gated");

    assert_eq!(error, OrganizationMemberRemovalError::PermissionDenied);
    assert_eq!(store.permission_check, Some((7, 9)));
    assert_eq!(store.removed_member, None);
}

fn command() -> OrganizationMemberRemovalCommand {
    OrganizationMemberRemovalCommand {
        actor_user_id: 7,
        organization_id: 9,
        target_user_id: 11,
    }
}

struct FakeOrganizationMemberRemovalStore {
    can_remove: bool,
    permission_check: Option<(i32, i32)>,
    removed_member: Option<(i32, i32)>,
}

impl FakeOrganizationMemberRemovalStore {
    fn allowed() -> Self {
        Self {
            can_remove: true,
            permission_check: None,
            removed_member: None,
        }
    }

    fn denied() -> Self {
        Self {
            can_remove: false,
            permission_check: None,
            removed_member: None,
        }
    }
}

impl AccessDecisionStore for FakeOrganizationMemberRemovalStore {
    type Error = OrganizationMemberRemovalError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRemovalError>> {
        assert_eq!(action.permission_name(), MANAGE_ORG_MEMBERS);
        let organization_id = match scope {
            AccessScope::Organization(scope) => scope.organization_id(),
            _ => panic!("member removal must use organization scope"),
        };
        self.permission_check = Some((actor.user_id, organization_id));
        ready(Ok(self.can_remove)).boxed()
    }
}

impl OrganizationMemberRemovalStore for FakeOrganizationMemberRemovalStore {
    fn remove_member(
        &mut self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>> {
        self.removed_member = Some((command.organization_id, command.target_user_id));
        ready(Ok(())).boxed()
    }
}
