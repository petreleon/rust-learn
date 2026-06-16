use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::manage_assessments::{
    self, AssessmentAuthoringCommand, AssessmentAuthoringError, AssessmentAuthoringUseCase,
    AssessmentUpdateCommand, AuthoredAssessmentOutput,
};
use crate::infra::postgres::learning::assessment_authoring_store::PostgresAssessmentAuthoringStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresAssessmentAuthoringUseCase {
    pool: DbPool,
}

impl PostgresAssessmentAuthoringUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl AssessmentAuthoringUseCase for PostgresAssessmentAuthoringUseCase {
    fn list_course_assessments_for_authoring(
        &self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentAuthoringStore::new(&mut conn);
            manage_assessments::list_course_assessments_for_authoring(
                &mut store,
                actor_user_id,
                course_id,
            )
            .await
        }
        .boxed()
    }

    fn create_assessment(
        &self,
        command: AssessmentAuthoringCommand,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentAuthoringStore::new(&mut conn);
            manage_assessments::create_assessment(&mut store, command).await
        }
        .boxed()
    }

    fn update_assessment(
        &self,
        command: AssessmentUpdateCommand,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresAssessmentAuthoringStore::new(&mut conn);
            manage_assessments::update_assessment(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresAssessmentAuthoringUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, AssessmentAuthoringError> {
        self.pool
            .get()
            .await
            .map_err(|error| AssessmentAuthoringError::Connection(error.to_string()))
    }
}
