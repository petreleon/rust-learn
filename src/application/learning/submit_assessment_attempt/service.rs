use futures::future::BoxFuture;

use crate::application::learning::submit_assessment_attempt::{
    AssessmentSubmissionError, SubmitAssessmentAttemptCommand, SubmitAssessmentAttemptOutput,
};

pub trait AssessmentSubmissionUseCase: Send + Sync {
    fn submit_assessment_attempt(
        &self,
        command: SubmitAssessmentAttemptCommand,
    ) -> BoxFuture<'_, Result<SubmitAssessmentAttemptOutput, AssessmentSubmissionError>>;
}
