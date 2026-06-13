use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::submit_assessment_attempt::{
    self, AssessmentSubmissionError, AssessmentSubmissionUseCase, SubmitAssessmentAttemptCommand,
    SubmitAssessmentAttemptOutput,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::assessment_submission_store::PostgresAssessmentSubmissionStore;

#[derive(Clone)]
pub struct PostgresAssessmentSubmissionUseCase {
    pool: DbPool,
}

impl PostgresAssessmentSubmissionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl AssessmentSubmissionUseCase for PostgresAssessmentSubmissionUseCase {
    fn submit_assessment_attempt(
        &self,
        command: SubmitAssessmentAttemptCommand,
    ) -> BoxFuture<'_, Result<SubmitAssessmentAttemptOutput, AssessmentSubmissionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentSubmissionStore::new(&mut conn);
            submit_assessment_attempt::submit_assessment_attempt(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresAssessmentSubmissionUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, AssessmentSubmissionError> {
        self.pool
            .get()
            .await
            .map_err(|error| AssessmentSubmissionError::Connection(error.to_string()))
    }
}
