use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentReadError,
};
use crate::application::learning::list_assessment_attempts::{self, AssessmentAttemptsUseCase};
use crate::application::learning::list_course_assessments::{self, CourseAssessmentsUseCase};
use crate::db::DbPool;
use crate::infra::postgres::learning::assessment_read_store::PostgresAssessmentReadStore;

#[derive(Clone)]
pub struct PostgresAssessmentReadUseCase {
    pool: DbPool,
}

impl PostgresAssessmentReadUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseAssessmentsUseCase for PostgresAssessmentReadUseCase {
    fn list_published_course_assessments(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentOutput>, AssessmentReadError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentReadStore::new(&mut conn);
            list_course_assessments::list_published_course_assessments(&mut store, course_id).await
        }
        .boxed()
    }
}

impl AssessmentAttemptsUseCase for PostgresAssessmentReadUseCase {
    fn list_user_assessment_attempts(
        &self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentAttemptOutput>, AssessmentReadError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentReadStore::new(&mut conn);
            list_assessment_attempts::list_user_assessment_attempts(
                &mut store,
                assessment_id,
                user_id,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresAssessmentReadUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, AssessmentReadError> {
        self.pool
            .get()
            .await
            .map_err(|error| AssessmentReadError::Connection(error.to_string()))
    }
}
