use futures::future::BoxFuture;

use crate::application::learning::update_course::{
    CourseUpdateError, CourseUpdateOutput, CourseUpdatePatch,
};

pub trait CourseUpdateStore {
    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseUpdateError>>;

    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseUpdateError>>;

    fn update_course(
        &mut self,
        course_id: i32,
        patch: CourseUpdatePatch,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>>;
}
