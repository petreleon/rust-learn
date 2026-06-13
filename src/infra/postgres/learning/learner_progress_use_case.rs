use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::learner_progress::{
    self, LearnerProgressError, LearnerProgressOutput, LearnerProgressUseCase,
    SaveLearnerProgressCommand,
};
use crate::db::DbPool;
use crate::infra::postgres::learning::learner_progress_store::PostgresLearnerProgressStore;

#[derive(Clone)]
pub struct PostgresLearnerProgressUseCase {
    pool: DbPool,
}

impl PostgresLearnerProgressUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl LearnerProgressUseCase for PostgresLearnerProgressUseCase {
    fn save_progress(
        &self,
        command: SaveLearnerProgressCommand,
    ) -> BoxFuture<'_, Result<LearnerProgressOutput, LearnerProgressError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLearnerProgressStore::new(&mut conn);
            learner_progress::save_learner_progress(&mut store, command).await
        }
        .boxed()
    }

    fn get_progress(
        &self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<LearnerProgressOutput>, LearnerProgressError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresLearnerProgressStore::new(&mut conn);
            learner_progress::get_learner_progress(&mut store, actor_user_id, course_id).await
        }
        .boxed()
    }
}

impl PostgresLearnerProgressUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, LearnerProgressError> {
        self.pool
            .get()
            .await
            .map_err(|error| LearnerProgressError::Connection(error.to_string()))
    }
}
