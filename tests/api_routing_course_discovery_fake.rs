use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery, CourseDiscoveryUseCase,
};

struct RouteOnlyCourseDiscoveryUseCase;

pub fn course_discovery_data() -> web::Data<Arc<dyn CourseDiscoveryUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseDiscoveryUseCase) as Arc<dyn CourseDiscoveryUseCase>)
}

impl CourseDiscoveryUseCase for RouteOnlyCourseDiscoveryUseCase {
    fn discover_courses(
        &self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>> {
        ready(Ok(CourseDiscoveryOutput {
            courses: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            organization_id: query.organization_id,
        }))
        .boxed()
    }
}
