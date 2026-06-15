use futures::future::{BoxFuture, FutureExt};

use crate::application::content::manage_chapter::{
    self, ChapterError, ChapterOutput, ChapterUseCases, CreateChapterCommand, UpdateChapterCommand,
};
use crate::infra::postgres::content::chapter_store::PostgresChapterStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresChapterUseCases {
    pool: DbPool,
}

impl PostgresChapterUseCases {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ChapterUseCases for PostgresChapterUseCases {
    fn list_chapters(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ChapterOutput>, ChapterError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresChapterStore::new(&mut conn);
            manage_chapter::list_chapters(&mut store, course_id).await
        }
        .boxed()
    }

    fn create_chapter(
        &self,
        command: CreateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresChapterStore::new(&mut conn);
            manage_chapter::create_chapter(&mut store, command).await
        }
        .boxed()
    }

    fn update_chapter(
        &self,
        chapter_id: i32,
        command: UpdateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresChapterStore::new(&mut conn);
            manage_chapter::update_chapter(&mut store, chapter_id, command).await
        }
        .boxed()
    }

    fn delete_chapter(&self, chapter_id: i32) -> BoxFuture<'_, Result<bool, ChapterError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresChapterStore::new(&mut conn);
            manage_chapter::delete_chapter(&mut store, chapter_id).await
        }
        .boxed()
    }
}

impl PostgresChapterUseCases {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        ChapterError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| ChapterError::Connection(error.to_string()))
    }
}
