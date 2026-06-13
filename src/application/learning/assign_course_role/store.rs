use futures::future::BoxFuture;

use crate::application::learning::assign_course_role::CourseRoleAssignmentError;

pub trait CourseRoleAssignmentStore {
    fn actor_min_level(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>>;

    fn target_min_level(
        &mut self,
        target_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>>;

    fn role_id_by_name(
        &mut self,
        role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>>;

    fn role_hierarchy_level(
        &mut self,
        role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>>;

    fn assign_role(
        &mut self,
        target_user_id: i32,
        course_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRoleAssignmentError>>;
}
