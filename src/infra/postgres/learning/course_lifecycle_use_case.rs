use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::update_course_lifecycle::{
    self, CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput,
    CourseLifecycleUseCase,
};
use crate::infra::postgres::learning::course_lifecycle_store::PostgresCourseLifecycleStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseLifecycleUseCase {
    pool: DbPool,
}

impl PostgresCourseLifecycleUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseLifecycleUseCase for PostgresCourseLifecycleUseCase {
    fn update_course_lifecycle(
        &self,
        command: CourseLifecycleCommand,
    ) -> BoxFuture<'_, Result<CourseLifecycleOutput, CourseLifecycleError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseLifecycleStore::new(&mut conn);
            update_course_lifecycle::update_course_lifecycle(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresCourseLifecycleUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseLifecycleError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseLifecycleError::Connection(error.to_string()))
    }
}
