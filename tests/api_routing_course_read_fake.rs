use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::get_course::{
    CourseOutput, CourseReadError, CourseReadUseCase,
};

struct RouteOnlyCourseReadUseCase;

pub fn course_read_data() -> web::Data<Arc<dyn CourseReadUseCase>> {
    web::Data::new(Arc::new(RouteOnlyCourseReadUseCase) as Arc<dyn CourseReadUseCase>)
}

impl CourseReadUseCase for RouteOnlyCourseReadUseCase {
    fn get_course(&self, _course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>> {
        ready(Ok(CourseOutput {
            id: 12,
            title: "Route smoke".to_string(),
            lifecycle_status: "draft".to_string(),
            description: None,
            topics: None,
            prerequisites: None,
        }))
        .boxed()
    }
}
