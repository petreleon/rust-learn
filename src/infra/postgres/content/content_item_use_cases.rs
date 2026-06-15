use futures::future::{BoxFuture, FutureExt};

use crate::application::content::manage_content_item::{
    self, ContentItemError, ContentItemOutput, ContentItemUseCases, CreateContentItemCommand,
    CreateContentItemOutput, UpdateContentItemCommand,
};
use crate::infra::postgres::content::content_item_store::PostgresContentItemStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresContentItemUseCases {
    pool: DbPool,
}

impl PostgresContentItemUseCases {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl ContentItemUseCases for PostgresContentItemUseCases {
    fn list_content_items(
        &self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ContentItemOutput>, ContentItemError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresContentItemStore::new(&mut conn);
            manage_content_item::list_content_items(&mut store, course_id, chapter_id).await
        }
        .boxed()
    }

    fn create_content_item(
        &self,
        course_id: i32,
        command: CreateContentItemCommand,
    ) -> BoxFuture<'_, Result<CreateContentItemOutput, ContentItemError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresContentItemStore::new(&mut conn);
            manage_content_item::create_content_item(&mut store, course_id, command).await
        }
        .boxed()
    }

    fn update_content_item(
        &self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
        command: UpdateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresContentItemStore::new(&mut conn);
            manage_content_item::update_content_item(
                &mut store, course_id, chapter_id, content_id, command,
            )
            .await
        }
        .boxed()
    }

    fn delete_content_item(
        &self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, ContentItemError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresContentItemStore::new(&mut conn);
            manage_content_item::delete_content_item(&mut store, course_id, chapter_id, content_id)
                .await
        }
        .boxed()
    }
}

impl PostgresContentItemUseCases {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        ContentItemError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| ContentItemError::Connection(error.to_string()))
    }
}
