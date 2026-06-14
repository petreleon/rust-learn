use futures::future::BoxFuture;

use crate::application::learning::create_course::{
    CourseCreationCommand, CourseCreationError, CourseCreationOutput,
};

pub trait CourseCreationUseCase: Send + Sync {
    fn create_course(
        &self,
        command: CourseCreationCommand,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>>;
}
