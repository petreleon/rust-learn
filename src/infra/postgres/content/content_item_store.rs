use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::manage_content_item::{
    ContentItemError, ContentItemOutput, CreateContentItemCommand, UpdateContentItemCommand,
};
use crate::application::content::ports::ContentItemStore;
use crate::infra::postgres::content::content_item_records::{
    create_content_item, delete_content_item, list_content_items, list_course_content_recipients,
    update_content_item,
};
use crate::infra::postgres::content::content_item_scope::{
    ensure_chapter_belongs_to_course, ensure_content_belongs_to_chapter,
};
use crate::infra::postgres::models::content::{NewContent, UpdateContent};

pub struct PostgresContentItemStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresContentItemStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ContentItemStore for PostgresContentItemStore<'_> {
    fn list_by_chapter(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ContentItemOutput>, ContentItemError>> {
        async move {
            ensure_chapter_belongs_to_course(self.conn, course_id, chapter_id).await?;
            list_content_items(self.conn, chapter_id).await
        }
        .boxed()
    }

    fn create(
        &mut self,
        course_id: i32,
        command: CreateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>> {
        async move {
            ensure_chapter_belongs_to_course(self.conn, course_id, command.chapter_id).await?;
            let new_content = NewContent {
                chapter_id: command.chapter_id,
                order: command.order,
                content_type: command.content_type,
                data: command.data,
            };

            create_content_item(self.conn, new_content).await
        }
        .boxed()
    }

    fn list_course_content_recipients(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, ContentItemError>> {
        async move { list_course_content_recipients(self.conn, course_id).await }.boxed()
    }

    fn update(
        &mut self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
        command: UpdateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>> {
        async move {
            ensure_chapter_belongs_to_course(self.conn, course_id, chapter_id).await?;
            ensure_content_belongs_to_chapter(self.conn, chapter_id, content_id).await?;
            let update = UpdateContent {
                order: command.order,
                content_type: command.content_type,
                data: command.data,
            };

            update_content_item(self.conn, content_id, update).await
        }
        .boxed()
    }

    fn delete(
        &mut self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, ContentItemError>> {
        async move {
            ensure_chapter_belongs_to_course(self.conn, course_id, chapter_id).await?;
            ensure_content_belongs_to_chapter(self.conn, chapter_id, content_id).await?;
            delete_content_item(self.conn, content_id).await
        }
        .boxed()
    }
}
