use futures::future::BoxFuture;

use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
};

pub trait OrganizationMemberRemovalUseCase: Send + Sync {
    fn remove_organization_member(
        &self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>>;
}
