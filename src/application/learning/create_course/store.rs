use futures::future::BoxFuture;

use crate::application::learning::create_course::{CourseCreationError, CourseCreationOutput};

pub trait CourseCreationStore {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseCreationError>>;

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseCreationError>>;

    fn create_course(
        &mut self,
        title: String,
        organization_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>>;
}
