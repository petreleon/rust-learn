use futures::future::BoxFuture;

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentQuestionForScoring, AssessmentReadError,
    CompletedAssessmentAttempt,
};
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;

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

pub trait AssessmentSubmissionStore {
    fn find_published_assessment(
        &mut self,
        course_id: i32,
        assessment_id: i32,
    ) -> BoxFuture<'_, Result<AssessmentOutput, AssessmentSubmissionError>>;

    fn count_completed_attempts(
        &mut self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<i64, AssessmentSubmissionError>>;

    fn list_questions_for_scoring(
        &mut self,
        assessment_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentQuestionForScoring>, AssessmentSubmissionError>>;

    fn create_completed_attempt(
        &mut self,
        attempt: CompletedAssessmentAttempt,
    ) -> BoxFuture<'_, Result<AssessmentAttemptOutput, AssessmentSubmissionError>>;
}
