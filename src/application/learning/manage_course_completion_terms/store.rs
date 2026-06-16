use futures::future::BoxFuture;

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsAuditEventOutput, CourseCompletionTermsCourseContext,
    CourseCompletionTermsDraft, CourseCompletionTermsError, CourseCompletionTermsOutput,
};

pub trait CourseCompletionTermsStore {
    fn course_context(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsCourseContext, CourseCompletionTermsError>>;

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseCompletionTermsError>>;

    fn create_terms(
        &mut self,
        draft: CourseCompletionTermsDraft,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn terms_by_id(
        &mut self,
        course_id: i32,
        terms_id: i64,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>;

    fn active_terms(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>;

    fn list_terms(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsOutput>, CourseCompletionTermsError>>;

    fn list_audit_events(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsAuditEventOutput>, CourseCompletionTermsError>>;

    fn activate_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn reject_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;

    fn withdraw_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>>;
}
