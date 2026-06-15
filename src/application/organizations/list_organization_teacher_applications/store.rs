use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError,
    OrganizationTeacherApplicationOrganizationOutput,
};

pub trait OrganizationTeacherApplicationListStore:
    AccessDecisionStore<Error = OrganizationTeacherApplicationListError>
{
    fn organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<
            OrganizationTeacherApplicationOrganizationOutput,
            OrganizationTeacherApplicationListError,
        >,
    >;

    fn list_teacher_applications(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError>,
    >;
}
