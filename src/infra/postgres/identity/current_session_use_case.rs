use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::current_session::{
    self, CurrentSessionError, CurrentSessionOutput, CurrentSessionUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::identity::current_session_store::PostgresCurrentSessionStore;

#[derive(Clone)]
pub struct PostgresCurrentSessionUseCase {
    pool: DbPool,
}

impl PostgresCurrentSessionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CurrentSessionUseCase for PostgresCurrentSessionUseCase {
    fn get_current_session(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<CurrentSessionOutput, CurrentSessionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresCurrentSessionStore::new(&mut conn);
            current_session::get_current_session(&mut store, user_id).await
        }
        .boxed()
    }
}

impl PostgresCurrentSessionUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        CurrentSessionError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| CurrentSessionError::Connection(error.to_string()))
    }
}
