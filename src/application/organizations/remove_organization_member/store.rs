use futures::future::BoxFuture;

use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
};

pub trait OrganizationMemberRemovalStore {
    fn can_remove_member(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRemovalError>>;

    fn remove_member(
        &mut self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>>;
}
