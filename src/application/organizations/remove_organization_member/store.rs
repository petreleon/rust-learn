use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
};

pub trait OrganizationMemberRemovalStore:
    AccessDecisionStore<Error = OrganizationMemberRemovalError>
{
    fn remove_member(
        &mut self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>>;
}
