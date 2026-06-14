use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::discover_courses::{
    self, CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery, CourseDiscoveryUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::course_discovery_store::PostgresCourseDiscoveryStore;

#[derive(Clone)]
pub struct PostgresCourseDiscoveryUseCase {
    pool: DbPool,
}

impl PostgresCourseDiscoveryUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CourseDiscoveryUseCase for PostgresCourseDiscoveryUseCase {
    fn discover_courses(
        &self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCourseDiscoveryStore::new(&mut conn);
            discover_courses::discover_courses(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresCourseDiscoveryUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, CourseDiscoveryError> {
        self.pool
            .get()
            .await
            .map_err(|error| CourseDiscoveryError::Connection(error.to_string()))
    }
}
