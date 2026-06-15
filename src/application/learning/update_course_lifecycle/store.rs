use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleError, CourseLifecycleOutput,
};

pub trait CourseLifecycleStore: AccessDecisionStore<Error = CourseLifecycleError> {
    fn update_status(
        &mut self,
        course_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>>;
}
