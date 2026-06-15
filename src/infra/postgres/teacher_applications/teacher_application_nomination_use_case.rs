use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    nominate_application::{
        self, TeacherApplicationNominationCommand, TeacherApplicationNominationError,
        TeacherApplicationNominationUseCase,
    },
    TeacherApplicationOutput,
};
use crate::infra::postgres::teacher_applications::teacher_application_nomination_store::PostgresTeacherApplicationNominationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresTeacherApplicationNominationUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationNominationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationNominationUseCase for PostgresTeacherApplicationNominationUseCase {
    fn nominate_application(
        &self,
        command: TeacherApplicationNominationCommand,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationNominationStore::new(&mut conn);
            nominate_application::nominate_application(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationNominationUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, TeacherApplicationNominationError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationNominationError::Connection(error.to_string()))
    }
}
