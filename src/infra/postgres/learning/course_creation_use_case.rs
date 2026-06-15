use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::create_course::{
    self, CourseCreationCommand, CourseCreationError, CourseCreationOutput, CourseCreationUseCase,
};
use crate::infra::postgres::learning::course_creation_store::PostgresCourseCreationStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseCreationUseCase {
    pool: DbPool,
}

impl PostgresCourseCreationUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseCreationUseCase for PostgresCourseCreationUseCase {
    fn create_course(
        &self,
        command: CourseCreationCommand,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseCreationStore::new(&mut conn);
            create_course::create_course(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresCourseCreationUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseCreationError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseCreationError::Connection(error.to_string()))
    }
}
