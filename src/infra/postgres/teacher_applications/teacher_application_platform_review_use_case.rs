use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::list_platform_review::{
    self, TeacherApplicationPlatformReviewError, TeacherApplicationPlatformReviewOutput,
    TeacherApplicationPlatformReviewQuery, TeacherApplicationPlatformReviewUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_store::PostgresTeacherApplicationPlatformReviewStore;

#[derive(Clone)]
pub struct PostgresTeacherApplicationPlatformReviewUseCase {
    pool: DbPool,
}

impl PostgresTeacherApplicationPlatformReviewUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TeacherApplicationPlatformReviewUseCase for PostgresTeacherApplicationPlatformReviewUseCase {
    fn list_platform_review_applications(
        &self,
        query: TeacherApplicationPlatformReviewQuery,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresTeacherApplicationPlatformReviewStore::new(&mut conn);
            list_platform_review::list_platform_review_applications(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresTeacherApplicationPlatformReviewUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, TeacherApplicationPlatformReviewError> {
        self.pool
            .get()
            .await
            .map_err(|error| TeacherApplicationPlatformReviewError::Connection(error.to_string()))
    }
}
