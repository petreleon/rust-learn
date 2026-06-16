use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::manage_course_completion_terms::{
    self, CourseCompletionTermsCounterCommand, CourseCompletionTermsDecisionCommand,
    CourseCompletionTermsError, CourseCompletionTermsHistoryOutput, CourseCompletionTermsListQuery,
    CourseCompletionTermsOutput, CourseCompletionTermsProposalCommand,
    CourseCompletionTermsUseCase,
};
use crate::infra::postgres::learning::course_completion_terms_store::PostgresCourseCompletionTermsStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseCompletionTermsUseCase {
    pool: DbPool,
}

impl PostgresCourseCompletionTermsUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseCompletionTermsUseCase for PostgresCourseCompletionTermsUseCase {
    fn list_course_completion_terms(
        &self,
        query: CourseCompletionTermsListQuery,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsHistoryOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::list_course_completion_terms(&mut store, query).await
        }
        .boxed()
    }

    fn submit_course_completion_terms(
        &self,
        command: CourseCompletionTermsProposalCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::submit_course_completion_terms(&mut store, command)
                .await
        }
        .boxed()
    }

    fn counter_course_completion_terms(
        &self,
        command: CourseCompletionTermsCounterCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::counter_course_completion_terms(&mut store, command)
                .await
        }
        .boxed()
    }

    fn accept_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::accept_course_completion_terms(&mut store, command)
                .await
        }
        .boxed()
    }

    fn reject_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::reject_course_completion_terms(&mut store, command)
                .await
        }
        .boxed()
    }

    fn withdraw_course_completion_terms(
        &self,
        command: CourseCompletionTermsDecisionCommand,
    ) -> BoxFuture<'_, Result<CourseCompletionTermsOutput, CourseCompletionTermsError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCompletionTermsStore::new(&mut conn);
            manage_course_completion_terms::withdraw_course_completion_terms(&mut store, command)
                .await
        }
        .boxed()
    }
}

impl PostgresCourseCompletionTermsUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        CourseCompletionTermsError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| CourseCompletionTermsError::Connection(error.to_string()))
    }
}
