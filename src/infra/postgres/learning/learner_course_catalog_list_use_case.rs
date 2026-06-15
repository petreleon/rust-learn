use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::application::learning::list_learner_course_catalog::{
    self, LearnerCourseCatalogListUseCase, LearnerCourseCatalogOutput, LearnerCourseCatalogQuery,
};
use crate::infra::postgres::learning::learner_course_catalog_list_store::PostgresLearnerCourseCatalogListStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresLearnerCourseCatalogListUseCase {
    pool: DbPool,
}

impl PostgresLearnerCourseCatalogListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl LearnerCourseCatalogListUseCase for PostgresLearnerCourseCatalogListUseCase {
    fn list_learner_course_catalog(
        &self,
        query: LearnerCourseCatalogQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseCatalogOutput, LearnerCourseCatalogError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLearnerCourseCatalogListStore::new(&mut conn);
            list_learner_course_catalog::list_learner_course_catalog(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresLearnerCourseCatalogListUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, LearnerCourseCatalogError> {
        self.pool
            .get()
            .await
            .map_err(|error| LearnerCourseCatalogError::Connection(error.to_string()))
    }
}
