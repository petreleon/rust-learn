use futures::future::BoxFuture;

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListOutput, OrganizationMemberListQuery,
};

pub trait OrganizationMemberListUseCase: Send + Sync {
    fn list_organization_members(
        &self,
        query: OrganizationMemberListQuery,
    ) -> BoxFuture<'_, Result<OrganizationMemberListOutput, OrganizationMemberListError>>;
}
