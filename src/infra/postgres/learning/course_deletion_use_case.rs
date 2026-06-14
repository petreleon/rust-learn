use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::delete_course::{
    self, CourseDeletionError, CourseDeletionOutcome, CourseDeletionUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::course_deletion_store::PostgresCourseDeletionStore;

#[derive(Clone)]
pub struct PostgresCourseDeletionUseCase {
    pool: DbPool,
}

impl PostgresCourseDeletionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseDeletionUseCase for PostgresCourseDeletionUseCase {
    fn delete_course(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<CourseDeletionOutcome, CourseDeletionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseDeletionStore::new(&mut conn);
            delete_course::delete_course(&mut store, course_id).await
        }
        .boxed()
    }
}

impl PostgresCourseDeletionUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseDeletionError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseDeletionError::Connection(error.to_string()))
    }
}
