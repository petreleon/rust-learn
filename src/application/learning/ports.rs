use futures::future::BoxFuture;

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentReadError,
};

pub trait AssessmentReadStore {
    fn list_published_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentOutput>, AssessmentReadError>>;

    fn list_attempts_for_user(
        &mut self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentAttemptOutput>, AssessmentReadError>>;
}
