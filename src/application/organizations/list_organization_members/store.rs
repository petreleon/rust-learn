use futures::future::BoxFuture;

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListOutput, OrganizationMemberListQuery,
};

pub trait OrganizationMemberListStore {
    fn list_organization_members(
        &mut self,
        query: OrganizationMemberListQuery,
    ) -> BoxFuture<'_, Result<OrganizationMemberListOutput, OrganizationMemberListError>>;
}
