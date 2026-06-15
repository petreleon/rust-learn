use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_course::{
    self, CourseOutput, CourseReadError, CourseReadUseCase,
};
use crate::infra::postgres::learning::course_read_store::PostgresCourseReadStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresCourseReadUseCase {
    pool: DbPool,
}

impl PostgresCourseReadUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseReadUseCase for PostgresCourseReadUseCase {
    fn get_course(&self, course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseReadStore::new(&mut conn);
            get_course::get_course(&mut store, course_id).await
        }
        .boxed()
    }
}

impl PostgresCourseReadUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseReadError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseReadError::Connection(error.to_string()))
    }
}
