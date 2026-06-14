use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::get_my_application::{
    self, TeacherApplicationSelfError, TeacherApplicationSelfOutput, TeacherApplicationSelfUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_self_store::PostgresTeacherApplicationSelfStore;

#[derive(Clone)]
pub struct PostgresTeacherApplicationSelfUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationSelfUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationSelfUseCase for PostgresTeacherApplicationSelfUseCase {
    fn get_my_application(
        &self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<TeacherApplicationSelfOutput, TeacherApplicationSelfError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationSelfStore::new(&mut conn);
            get_my_application::get_my_application(&mut store, actor_user_id).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationSelfUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, TeacherApplicationSelfError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationSelfError::Connection(error.to_string()))
    }
}
