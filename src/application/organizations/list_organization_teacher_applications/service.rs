use futures::future::BoxFuture;

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListOutput,
    OrganizationTeacherApplicationListQuery,
};

pub trait OrganizationTeacherApplicationListUseCase: Send + Sync {
    fn list_organization_teacher_applications(
        &self,
        query: OrganizationTeacherApplicationListQuery,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationListOutput, OrganizationTeacherApplicationListError>,
    >;
}
