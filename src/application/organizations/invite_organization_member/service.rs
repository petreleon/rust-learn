use futures::future::BoxFuture;

use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteCommand, OrganizationMemberInviteError, OrganizationMemberInviteOutput,
};

pub trait OrganizationMemberInviteUseCase: Send + Sync {
    fn invite_organization_member(
        &self,
        command: OrganizationMemberInviteCommand,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteOutput, OrganizationMemberInviteError>>;
}
