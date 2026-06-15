use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::learning::create_course::{CourseCreationError, CourseCreationOutput};

pub trait CourseCreationStore: AccessDecisionStore<Error = CourseCreationError> {
    fn create_course(
        &mut self,
        title: String,
        organization_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>>;
}
