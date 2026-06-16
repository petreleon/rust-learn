use futures::future::BoxFuture;

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsCounterCommand, CourseCompletionTermsDecisionCommand,
    CourseCompletionTermsError, CourseCompletionTermsHistoryOutput, CourseCompletionTermsListQuery,
    CourseCompletionTermsOutput, CourseCompletionTermsProposalCommand,
};

pub trait CourseCompletionTermsUseCase: Send + Sync {
    fn list_course_completion_terms(
        &self,
        query: CourseCompletionTermsListQuery,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsHistoryOutput, CourseCompletionTermsError>>;

    fn submit_course_completion_terms(
        &self,
        command: CourseCompletionTermsProposalCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn counter_course_completion_terms(
        &self,
        command: CourseCompletionTermsCounterCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn accept_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn reject_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn withdraw_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;
}
