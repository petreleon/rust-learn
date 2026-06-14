use futures::future::BoxFuture;

use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput,
};

pub trait CourseLifecycleUseCase: Send + Sync {
    fn update_course_lifecycle(
        &self,
        command: CourseLifecycleCommand,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>>;
}
