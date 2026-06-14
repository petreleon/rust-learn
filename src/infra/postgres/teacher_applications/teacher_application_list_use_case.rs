use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    list_applications::{
        self, TeacherApplicationListError, TeacherApplicationListQuery,
        TeacherApplicationListUseCase,
    },
    TeacherApplicationOutput,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_list_store::PostgresTeacherApplicationListStore;

#[derive(Clone)]
pub struct PostgresTeacherApplicationListUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationListUseCase for PostgresTeacherApplicationListUseCase {
    fn list_applications(
        &self,
        query: TeacherApplicationListQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationListStore::new(&mut conn);
            list_applications::list_applications(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationListUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherApplicationListError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationListError::Connection(error.to_string()))
    }
}
