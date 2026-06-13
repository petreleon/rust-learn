use futures::future::BoxFuture;

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
};

pub trait OrganizationCourseListUseCase: Send + Sync {
    fn list_organization_courses(
        &self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>>;
}
