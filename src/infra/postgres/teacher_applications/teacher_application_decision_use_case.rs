use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    decide_application::{
        self, TeacherApplicationDecisionCommand, TeacherApplicationDecisionError,
        TeacherApplicationDecisionUseCase,
    },
    TeacherApplicationOutput,
};
use crate::infra::postgres::teacher_applications::teacher_application_decision_store::PostgresTeacherApplicationDecisionStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresTeacherApplicationDecisionUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationDecisionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationDecisionUseCase for PostgresTeacherApplicationDecisionUseCase {
    fn decide_application(
        &self,
        command: TeacherApplicationDecisionCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationDecisionStore::new(&mut conn);
            decide_application::decide_application(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationDecisionUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, TeacherApplicationDecisionError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationDecisionError::Connection(error.to_string()))
    }
}
