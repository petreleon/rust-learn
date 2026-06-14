use futures::future::BoxFuture;

use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteError, OrganizationMemberInviteTarget,
};

pub trait OrganizationMemberInviteStore {
    fn can_invite_member(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberInviteError>>;

    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteTarget, OrganizationMemberInviteError>>;

    fn assign_member_role(
        &mut self,
        actor_user_id: i32,
        target_user_id: i32,
        organization_id: i32,
        role_name: String,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberInviteError>>;
}
