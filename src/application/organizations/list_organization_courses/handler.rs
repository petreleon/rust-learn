use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
    OrganizationCourseListStore,
};

pub async fn list_organization_courses(
    store: &mut impl OrganizationCourseListStore,
    query: OrganizationCourseListQuery,
) -> Result<OrganizationCourseListOutput, OrganizationCourseListError> {
    store.list_organization_courses(query).await
}
