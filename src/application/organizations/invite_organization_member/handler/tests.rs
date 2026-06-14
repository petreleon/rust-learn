use futures::executor::block_on;
use futures::future::{ready, BoxFuture, FutureExt};

use super::invite_organization_member;
use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteCommand, OrganizationMemberInviteError, OrganizationMemberInviteStore,
    OrganizationMemberInviteTarget,
};

#[test]
fn invites_member_after_permission_and_lookup() {
    let mut store = FakeOrganizationMemberInviteStore::allowed();

    let output = block_on(invite_organization_member(&mut store, command()))
        .expect("member invite should succeed");

    assert_eq!(store.permission_check, Some((7, 9)));
    assert_eq!(store.lookup_email, Some("learner@example.com".to_string()));
    assert_eq!(store.assigned_role, Some((7, 11, 9, "STUDENT".to_string())));
    assert_eq!(output.user_id, 11);
    assert_eq!(output.role, "STUDENT");
}

#[test]
fn rejects_without_lookup_when_permission_denied() {
    let mut store = FakeOrganizationMemberInviteStore::denied();

    let error = block_on(invite_organization_member(&mut store, command()))
        .expect_err("member invite should be permission gated");

    assert_eq!(error, OrganizationMemberInviteError::PermissionDenied);
    assert_eq!(store.permission_check, Some((7, 9)));
    assert_eq!(store.lookup_email, None);
    assert_eq!(store.assigned_role, None);
}

fn command() -> OrganizationMemberInviteCommand {
    OrganizationMemberInviteCommand {
        actor_user_id: 7,
        organization_id: 9,
        email: " learner@example.com ".to_string(),
        role_name: None,
    }
}

struct FakeOrganizationMemberInviteStore {
    can_invite: bool,
    permission_check: Option<(i32, i32)>,
    lookup_email: Option<String>,
    assigned_role: Option<(i32, i32, i32, String)>,
}

impl FakeOrganizationMemberInviteStore {
    fn allowed() -> Self {
        Self {
            can_invite: true,
            permission_check: None,
            lookup_email: None,
            assigned_role: None,
        }
    }

    fn denied() -> Self {
        Self {
            can_invite: false,
            permission_check: None,
            lookup_email: None,
            assigned_role: None,
        }
    }
}

impl OrganizationMemberInviteStore for FakeOrganizationMemberInviteStore {
    fn can_invite_member(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberInviteError>> {
        self.permission_check = Some((actor_user_id, organization_id));
        ready(Ok(self.can_invite)).boxed()
    }

    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteTarget, OrganizationMemberInviteError>> {
        self.lookup_email = Some(email);
        ready(Ok(OrganizationMemberInviteTarget {
            user_id: 11,
            name: "Learner One".to_string(),
            email: "learner@example.com".to_string(),
        }))
        .boxed()
    }

    fn assign_member_role(
        &mut self,
        actor_user_id: i32,
        target_user_id: i32,
        organization_id: i32,
        role_name: String,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberInviteError>> {
        self.assigned_role = Some((actor_user_id, target_user_id, organization_id, role_name));
        ready(Ok(())).boxed()
    }
}
