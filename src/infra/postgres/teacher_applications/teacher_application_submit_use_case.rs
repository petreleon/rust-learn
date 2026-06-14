use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    submit_application::{
        self, TeacherApplicationSubmitCommand, TeacherApplicationSubmitError,
        TeacherApplicationSubmitUseCase,
    },
    TeacherApplicationOutput,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_submit_store::PostgresTeacherApplicationSubmitStore;

#[derive(Clone)]
pub struct PostgresTeacherApplicationSubmitUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationSubmitUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationSubmitUseCase for PostgresTeacherApplicationSubmitUseCase {
    fn submit_application(
        &self,
        command: TeacherApplicationSubmitCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationSubmitError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationSubmitStore::new(&mut conn);
            submit_application::submit_application(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationSubmitUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherApplicationSubmitError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationSubmitError::Connection(error.to_string()))
    }
}
