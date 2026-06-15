use futures::future::{BoxFuture, FutureExt};

use super::{assign_organization_member_role, ASSIGN_ROLES_TO_ORG_USERS};
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentStore,
};

#[derive(Default)]
struct FakeOrganizationMemberRoleAssignmentStore {
    can_assign: bool,
    actor_level: Option<i32>,
    target_level: Option<i32>,
    role_id: Option<i32>,
    role_level: Option<i32>,
    assigned: Option<(i32, i32, i32)>,
    recorded: Option<(i32, i32, i32, String)>,
}

impl AccessDecisionStore for FakeOrganizationMemberRoleAssignmentStore {
    type Error = OrganizationMemberRoleAssignmentError;

    fn can(
        &mut self,
        _actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRoleAssignmentError>> {
        assert_eq!(action.permission_name(), ASSIGN_ROLES_TO_ORG_USERS);
        assert!(matches!(scope, AccessScope::Organization(_)));
        let can_assign = self.can_assign;
        async move { Ok(can_assign) }.boxed()
    }
}

impl OrganizationMemberRoleAssignmentStore for FakeOrganizationMemberRoleAssignmentStore {
    fn actor_min_level(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        let level = self.actor_level;
        async move { Ok(level) }.boxed()
    }

    fn target_min_level(
        &mut self,
        _target_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        let level = self.target_level;
        async move { Ok(level) }.boxed()
    }

    fn role_id_by_name(
        &mut self,
        _role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        let role_id = self.role_id;
        async move { Ok(role_id) }.boxed()
    }

    fn role_hierarchy_level(
        &mut self,
        _role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        let level = self.role_level;
        async move { Ok(level) }.boxed()
    }

    fn assign_role(
        &mut self,
        target_user_id: i32,
        organization_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>> {
        self.assigned = Some((target_user_id, organization_id, role_id));
        async move { Ok(()) }.boxed()
    }

    fn record_role_assignment(
        &mut self,
        command: &OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>> {
        self.recorded = Some((
            command.actor_user_id,
            command.target_user_id,
            command.organization_id,
            command.role_name.clone(),
        ));
        async move { Ok(()) }.boxed()
    }
}

#[tokio::test]
async fn assigns_role_after_permission_and_hierarchy_checks() {
    let mut store = FakeOrganizationMemberRoleAssignmentStore {
        can_assign: true,
        actor_level: Some(1),
        role_id: Some(23),
        role_level: Some(4),
        ..Default::default()
    };

    let output = assign_organization_member_role(&mut store, command("STUDENT"))
        .await
        .expect("higher actor should assign lower role");

    assert_eq!(output.role_name, "STUDENT");
    assert_eq!(store.assigned, Some((9, 17, 23)));
    assert_eq!(store.recorded, Some((5, 9, 17, "STUDENT".to_string())));
}

#[tokio::test]
async fn rejects_without_assign_permission() {
    let mut store = FakeOrganizationMemberRoleAssignmentStore::default();

    let error = assign_organization_member_role(&mut store, command("STUDENT"))
        .await
        .expect_err("assignment should be permission gated");

    assert_eq!(
        error,
        OrganizationMemberRoleAssignmentError::PermissionDenied
    );
    assert_eq!(store.assigned, None);
    assert_eq!(store.recorded, None);
}

#[tokio::test]
async fn rejects_equal_or_higher_target_role() {
    let mut store = FakeOrganizationMemberRoleAssignmentStore {
        can_assign: true,
        actor_level: Some(2),
        target_level: Some(1),
        role_id: Some(23),
        role_level: Some(4),
        ..Default::default()
    };

    let error = assign_organization_member_role(&mut store, command("STUDENT"))
        .await
        .expect_err("actor cannot modify higher target");

    assert_eq!(
        error,
        OrganizationMemberRoleAssignmentError::HierarchyViolation
    );
    assert_eq!(store.assigned, None);
}

fn command(role_name: &str) -> OrganizationMemberRoleAssignmentCommand {
    OrganizationMemberRoleAssignmentCommand {
        actor_user_id: 5,
        organization_id: 17,
        target_user_id: 9,
        role_name: role_name.to_string(),
    }
}
