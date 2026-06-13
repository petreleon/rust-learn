use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdateUseCase,
};

struct RouteOnlyCourseUpdateUseCase;

pub fn course_update_data() -> web::Data<Arc<dyn CourseUpdateUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseUpdateUseCase) as Arc<dyn CourseUpdateUseCase>)
}

impl CourseUpdateUseCase for RouteOnlyCourseUpdateUseCase {
    fn update_course(
        &self,
        command: CourseUpdateCommand,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>> {
        ready(Ok(CourseUpdateOutput {
            id: command.course_id,
            title: command.title.unwrap_or_else(|| "Route smoke".to_string()),
            lifecycle_status: "draft".to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        }))
        .boxed()
    }
}
