use futures::future::BoxFuture;

use crate::application::learning::assessment::{AssessmentAttemptOutput, AssessmentReadError};

pub trait AssessmentAttemptsUseCase: Send + Sync {
    fn list_user_assessment_attempts(
        &self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentAttemptOutput>, AssessmentReadError>>;
}
