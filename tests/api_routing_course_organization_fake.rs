use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::list_course_organizations::{
    CourseOrganizationOutput, CourseOrganizationReadError, CourseOrganizationsUseCase,
};

struct RouteOnlyCourseOrganizationsUseCase;

pub fn course_organizations_data() -> web::Data<Arc<dyn CourseOrganizationsUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyCourseOrganizationsUseCase) as Arc<dyn CourseOrganizationsUseCase>
    )
}

impl CourseOrganizationsUseCase for RouteOnlyCourseOrganizationsUseCase {
    fn list_course_organizations(
        &self,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>> {
        ready(Ok(Vec::new())).boxed()
    }
}
