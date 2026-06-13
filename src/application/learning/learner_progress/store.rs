use futures::future::BoxFuture;

use crate::application::learning::learner_progress::{LearnerProgressError, LearnerProgressOutput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressCourse {
    pub id: i32,
    pub lifecycle_status: String,
}

pub trait LearnerProgressStore {
    fn course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<ProgressCourse, LearnerProgressError>>;

    fn course_organization_ids(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, LearnerProgressError>>;

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>>;

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>>;

    fn actor_course_roles(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<String>, LearnerProgressError>>;

    fn has_join_request_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        status: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>>;

    fn content_belongs_to_course(
        &mut self,
        course_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>>;

    fn save_progress(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<LearnerProgressOutput, LearnerProgressError>>;

    fn get_progress(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<LearnerProgressOutput>, LearnerProgressError>>;
}
