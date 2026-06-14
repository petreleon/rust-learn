use futures::future::BoxFuture;

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
};

pub trait OrganizationCourseListStore {
    fn list_organization_courses(
        &mut self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>>;
}
