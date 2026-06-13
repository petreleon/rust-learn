use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::list_reward_history::{
    self, StudentRewardHistoryEntry, StudentRewardHistoryError, StudentRewardHistoryQuery,
    StudentRewardHistoryUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::rewards::reward_history_store::PostgresStudentRewardHistoryStore;

#[derive(Clone)]
pub struct PostgresStudentRewardHistoryUseCase {
    pool: DbPool,
}

impl PostgresStudentRewardHistoryUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl StudentRewardHistoryUseCase for PostgresStudentRewardHistoryUseCase {
    fn list_student_reward_history(
        &self,
        actor_user_id: i32,
        query: StudentRewardHistoryQuery,
    ) -> BoxFuture<'_, Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresStudentRewardHistoryStore::new(&mut conn);
            list_reward_history::list_student_reward_history(&mut store, actor_user_id, query).await
        }
        .boxed()
    }
}

impl PostgresStudentRewardHistoryUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        StudentRewardHistoryError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| StudentRewardHistoryError::Connection(error.to_string()))
    }
}
