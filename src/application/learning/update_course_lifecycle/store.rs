use futures::future::BoxFuture;

use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleError, CourseLifecycleOutput,
};

pub trait CourseLifecycleStore {
    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>>;

    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseLifecycleError>>;

    fn update_status(
        &mut self,
        course_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>>;
}
