use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::create_course::{
    CourseCreationCommand, CourseCreationError, CourseCreationOutput, CourseCreationUseCase,
};

struct RouteOnlyCourseCreationUseCase;

pub fn course_creation_data() -> web::Data<Arc<dyn CourseCreationUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseCreationUseCase) as Arc<dyn CourseCreationUseCase>)
}

impl CourseCreationUseCase for RouteOnlyCourseCreationUseCase {
    fn create_course(
        &self,
        command: CourseCreationCommand,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>> {
        ready(Ok(CourseCreationOutput {
            id: 12,
            title: command.title,
            lifecycle_status: "draft".to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        }))
        .boxed()
    }
}
