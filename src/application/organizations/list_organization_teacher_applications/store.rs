use futures::future::BoxFuture;

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError,
    OrganizationTeacherApplicationOrganizationOutput,
};

pub trait OrganizationTeacherApplicationListStore {
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

    fn can_view_applications(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>>;

    fn can_nominate_teachers(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>>;

    fn list_teacher_applications(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError>,
    >;
}
