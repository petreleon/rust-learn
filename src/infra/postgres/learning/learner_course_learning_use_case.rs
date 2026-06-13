use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_learner_course_learning::{
    self, LearnerCourseLearningError, LearnerCourseLearningOutput, LearnerCourseLearningQuery,
    LearnerCourseLearningUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::learner_course_learning_store::PostgresLearnerCourseLearningStore;

#[derive(Clone)]
pub struct PostgresLearnerCourseLearningUseCase {
    pool: DbPool,
}

impl PostgresLearnerCourseLearningUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl LearnerCourseLearningUseCase for PostgresLearnerCourseLearningUseCase {
    fn get_learner_course_learning(
        &self,
        query: LearnerCourseLearningQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseLearningOutput, LearnerCourseLearningError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLearnerCourseLearningStore::new(&mut conn);
            get_learner_course_learning::get_learner_course_learning(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresLearnerCourseLearningUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, LearnerCourseLearningError> {
        self.pool
            .get()
            .await
            .map_err(|error| LearnerCourseLearningError::Connection(error.to_string()))
    }
}
