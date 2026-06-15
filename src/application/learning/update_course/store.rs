use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::learning::update_course::{
    CourseUpdateError, CourseUpdateOutput, CourseUpdatePatch,
};

pub trait CourseUpdateStore: AccessDecisionStore<Error = CourseUpdateError> {
    fn update_course(
        &mut self,
        course_id: i32,
        patch: CourseUpdatePatch,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>>;
}
