use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListOutput, OrganizationMemberListQuery,
    OrganizationMemberListStore,
};

pub async fn list_organization_members(
    store: &mut impl OrganizationMemberListStore,
    query: OrganizationMemberListQuery,
) -> Result<OrganizationMemberListOutput, OrganizationMemberListError> {
    store.list_organization_members(query).await
}
