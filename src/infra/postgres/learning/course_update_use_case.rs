use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::update_course::{
    self, CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdateUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::course_update_store::PostgresCourseUpdateStore;

#[derive(Clone)]
pub struct PostgresCourseUpdateUseCase {
    pool: DbPool,
}

impl PostgresCourseUpdateUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseUpdateUseCase for PostgresCourseUpdateUseCase {
    fn update_course(
        &self,
        command: CourseUpdateCommand,
    ) -> BoxFuture<'_, Result<CourseUpdateOutput, CourseUpdateError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseUpdateStore::new(&mut conn);
            update_course::update_course(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresCourseUpdateUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseUpdateError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseUpdateError::Connection(error.to_string()))
    }
}
