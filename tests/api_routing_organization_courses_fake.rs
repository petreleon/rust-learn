use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
    OrganizationCourseListUseCase, OrganizationCourseSummaryOutput,
};

struct RouteOnlyOrganizationCourseListUseCase;

pub fn organization_course_list_data() -> web::Data<Arc<dyn OrganizationCourseListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationCourseListUseCase) as Arc<dyn OrganizationCourseListUseCase>
    )
}

impl OrganizationCourseListUseCase for RouteOnlyOrganizationCourseListUseCase {
    fn list_organization_courses(
        &self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>> {
        ready(Ok(OrganizationCourseListOutput {
            organization: OrganizationCourseSummaryOutput {
                id: query.organization_id,
                name: "Route smoke".to_string(),
            },
            courses: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            lifecycle_status: query.lifecycle_status,
            reward_available: query.reward_available,
        }))
        .boxed()
    }
}
