use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsAuditEventOutput, CourseCompletionTermsCourseContext,
    CourseCompletionTermsDraft, CourseCompletionTermsError, CourseCompletionTermsOutput,
    CourseCompletionTermsStore,
};
use crate::infra::postgres::learning::{
    course_completion_terms_operations, course_completion_terms_permissions,
    course_completion_terms_records,
};

pub struct PostgresCourseCompletionTermsStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseCompletionTermsStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseCompletionTermsStore for PostgresCourseCompletionTermsStore<'_> {
    fn course_context(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsCourseContext, CourseCompletionTermsError>> {
        async move { course_completion_terms_records::course_context(self.conn, course_id).await }
            .boxed()
    }

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseCompletionTermsError>> {
        let permission = permission.to_string();
        async move {
            course_completion_terms_permissions::has_course_permission(
                self.conn,
                actor_user_id,
                course_id,
                &permission,
            )
            .await
        }
        .boxed()
    }

    fn create_terms(
        &mut self,
        draft: CourseCompletionTermsDraft,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move { course_completion_terms_operations::create_terms(self.conn, draft).await }
            .boxed()
    }

    fn terms_by_id(
        &mut self,
        course_id: i32,
        terms_id: i64,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>
    {
        async move {
            course_completion_terms_records::terms_by_id(self.conn, course_id, terms_id)
                .await?
                .map(CourseCompletionTermsOutput::try_from)
                .transpose()
        }
        .boxed()
    }

    fn active_terms(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<CourseCompletionTermsOutput>, CourseCompletionTermsError>>
    {
        async move {
            course_completion_terms_records::active_terms(self.conn, course_id)
                .await?
                .map(CourseCompletionTermsOutput::try_from)
                .transpose()
        }
        .boxed()
    }

    fn list_terms(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsOutput>, CourseCompletionTermsError>> {
        async move {
            course_completion_terms_records::list_terms(self.conn, course_id)
                .await?
                .into_iter()
                .map(CourseCompletionTermsOutput::try_from)
                .collect()
        }
        .boxed()
    }

    fn list_audit_events(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseCompletionTermsAuditEventOutput>, CourseCompletionTermsError>>
    {
        async move {
            course_completion_terms_records::list_audit_events(self.conn, course_id)
                .await?
                .into_iter()
                .map(CourseCompletionTermsAuditEventOutput::try_from)
                .collect()
        }
        .boxed()
    }

    fn activate_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            course_completion_terms_operations::activate_terms(
                self.conn,
                terms_id,
                actor_user_id,
                note,
            )
            .await
        }
        .boxed()
    }

    fn reject_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            course_completion_terms_operations::transition_terms(
                self.conn,
                terms_id,
                actor_user_id,
                note,
                false,
            )
            .await
        }
        .boxed()
    }

    fn withdraw_terms(
        &mut self,
        terms_id: i64,
        actor_user_id: i32,
        note: Option<String>,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            course_completion_terms_operations::transition_terms(
                self.conn,
                terms_id,
                actor_user_id,
                note,
                true,
            )
            .await
        }
        .boxed()
    }
}
