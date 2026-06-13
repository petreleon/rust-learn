use futures::future::BoxFuture;

use crate::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput,
};

pub trait CourseUpdateUseCase: Send + Sync {
    fn update_course(
        &self,
        command: CourseUpdateCommand,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>>;
}
