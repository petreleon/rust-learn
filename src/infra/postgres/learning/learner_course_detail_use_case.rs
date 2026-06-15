use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_learner_course_detail::{
    self, LearnerCourseDetailOutput, LearnerCourseDetailQuery, LearnerCourseDetailUseCase,
};
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::infra::postgres::learning::learner_course_detail_store::PostgresLearnerCourseDetailStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresLearnerCourseDetailUseCase {
    pool: DbPool,
}

impl PostgresLearnerCourseDetailUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl LearnerCourseDetailUseCase for PostgresLearnerCourseDetailUseCase {
    fn get_learner_course_detail(
        &self,
        query: LearnerCourseDetailQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseDetailOutput, LearnerCourseCatalogError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLearnerCourseDetailStore::new(&mut conn);
            get_learner_course_detail::get_learner_course_detail(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresLearnerCourseDetailUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, LearnerCourseCatalogError> {
        self.pool
            .get()
            .await
            .map_err(|error| LearnerCourseCatalogError::Connection(error.to_string()))
    }
}
