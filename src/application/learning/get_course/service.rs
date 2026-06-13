use futures::future::BoxFuture;

use crate::application::learning::get_course::{CourseOutput, CourseReadError};

pub trait CourseReadUseCase: Send + Sync {
    fn get_course(&self, course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>>;
}
