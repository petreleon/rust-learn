use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleUseCase,
};

struct RouteOnlyCourseLifecycleUseCase;

pub fn course_lifecycle_data() -> web::Data<Arc<dyn CourseLifecycleUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseLifecycleUseCase) as Arc<dyn CourseLifecycleUseCase>)
}

impl CourseLifecycleUseCase for RouteOnlyCourseLifecycleUseCase {
    fn update_course_lifecycle(
        &self,
        command: CourseLifecycleCommand,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>> {
        ready(Ok(CourseLifecycleOutput {
            id: command.course_id,
            title: "Route smoke".to_string(),
            lifecycle_status: command.status,
            description: None,
            topics: None,
            prerequisites: None,
        }))
        .boxed()
    }
}
