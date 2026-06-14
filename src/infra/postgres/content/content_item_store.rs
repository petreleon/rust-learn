use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::manage_content_item::{
    ContentItemError, ContentItemOutput, CreateContentItemCommand, UpdateContentItemCommand,
};
use crate::application::content::ports::ContentItemStore;
use crate::db::schema::{chapters, contents, user_role_course};
use crate::infra::postgres::content::mappers::content_item_output_from_record;
use crate::models::content::{Content, NewContent, UpdateContent};

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
            contents::table
                .filter(contents::chapter_id.eq(chapter_id))
                .order(contents::order.asc())
                .load::<Content>(self.conn)
                .await
                .map(|items| {
                    items
                        .into_iter()
                        .map(content_item_output_from_record)
                        .collect()
                })
                .map_err(map_database_error)
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

            diesel::insert_into(contents::table)
                .values(&new_content)
                .get_result::<Content>(self.conn)
                .await
                .map(content_item_output_from_record)
                .map_err(map_database_error)
        }
        .boxed()
    }

    fn list_course_content_recipients(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, ContentItemError>> {
        async move {
            user_role_course::table
                .filter(user_role_course::course_id.eq(course_id))
                .select(user_role_course::user_id)
                .distinct()
                .load::<Option<i32>>(self.conn)
                .await
                .map(|ids| ids.into_iter().flatten().collect())
                .map_err(map_database_error)
        }
        .boxed()
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

            diesel::update(contents::table.find(content_id))
                .set(&update)
                .get_result::<Content>(self.conn)
                .await
                .map(content_item_output_from_record)
                .map_err(map_content_error)
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
            diesel::delete(contents::table.find(content_id))
                .execute(self.conn)
                .await
                .map(|count| count > 0)
                .map_err(map_database_error)
        }
        .boxed()
    }
}

async fn ensure_chapter_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    chapter_id: i32,
) -> Result<(), ContentItemError> {
    chapters::table
        .filter(chapters::id.eq(chapter_id))
        .filter(chapters::course_id.eq(course_id))
        .select(chapters::id)
        .first::<i32>(conn)
        .await
        .map(|_| ())
        .map_err(map_chapter_error)
}

async fn ensure_content_belongs_to_chapter(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_id: i32,
) -> Result<(), ContentItemError> {
    contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .select(contents::id)
        .first::<i32>(conn)
        .await
        .map(|_| ())
        .map_err(map_content_error)
}

fn map_chapter_error(error: diesel::result::Error) -> ContentItemError {
    match error {
        diesel::result::Error::NotFound => ContentItemError::ChapterNotFound,
        other => ContentItemError::Database(other.to_string()),
    }
}

fn map_content_error(error: diesel::result::Error) -> ContentItemError {
    match error {
        diesel::result::Error::NotFound => ContentItemError::ContentNotFound,
        other => ContentItemError::Database(other.to_string()),
    }
}

fn map_database_error(error: diesel::result::Error) -> ContentItemError {
    ContentItemError::Database(error.to_string())
}
