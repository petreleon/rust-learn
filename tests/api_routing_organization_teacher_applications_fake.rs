use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListOutput,
    OrganizationTeacherApplicationListQuery, OrganizationTeacherApplicationListUseCase,
    OrganizationTeacherApplicationOrganizationOutput,
    OrganizationTeacherApplicationPermissionsOutput, TeacherApplicationDashboardSummaryOutput,
};

struct RouteOnlyOrganizationTeacherApplicationListUseCase;

pub fn organization_teacher_application_data(
) -> web::Data<Arc<dyn OrganizationTeacherApplicationListUseCase>> {
    web::Data::new(Arc::new(RouteOnlyOrganizationTeacherApplicationListUseCase)
        as Arc<dyn OrganizationTeacherApplicationListUseCase>)
}

impl OrganizationTeacherApplicationListUseCase
    for RouteOnlyOrganizationTeacherApplicationListUseCase
{
    fn list_organization_teacher_applications(
        &self,
        query: OrganizationTeacherApplicationListQuery,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationListOutput, OrganizationTeacherApplicationListError>,
    > {
        ready(Ok(OrganizationTeacherApplicationListOutput {
            organization: OrganizationTeacherApplicationOrganizationOutput {
                id: query.organization_id,
                name: "Route smoke".to_string(),
            },
            applications: Vec::new(),
            summary: TeacherApplicationDashboardSummaryOutput::default(),
            operator_permissions: OrganizationTeacherApplicationPermissionsOutput {
                can_view_applications: true,
                can_nominate_teachers: false,
            },
            total: 0,
            limit: query.limit.unwrap_or(25),
            offset: query.offset.unwrap_or(0),
            status: query.status,
            search: query.search,
        }))
        .boxed()
    }
}
