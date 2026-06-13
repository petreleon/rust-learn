use futures::future::BoxFuture;

use crate::application::learning::get_course::{CourseOutput, CourseReadError};

pub trait CourseReadStore {
    fn get(&mut self, course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>>;
}
