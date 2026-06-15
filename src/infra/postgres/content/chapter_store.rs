use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::manage_chapter::{
    ChapterError, ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};
use crate::application::content::ports::ChapterStore;
use crate::db::schema::chapters;
use crate::infra::postgres::content::mappers::chapter_output_from_record;
use crate::models::chapter::{Chapter, NewChapter, UpdateChapter};

pub struct PostgresChapterStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresChapterStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ChapterStore for PostgresChapterStore<'_> {
    fn list_by_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ChapterOutput>, ChapterError>> {
        async move {
            chapters::table
                .filter(chapters::course_id.eq(course_id))
                .order(chapters::order.asc())
                .load::<Chapter>(self.conn)
                .await
                .map(|chapters| {
                    chapters
                        .into_iter()
                        .map(chapter_output_from_record)
                        .collect()
                })
                .map_err(map_chapter_error)
        }
        .boxed()
    }

    fn create(
        &mut self,
        command: CreateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>> {
        async move {
            let new_chapter = NewChapter {
                course_id: command.course_id,
                title: command.title,
                order: command.order,
            };

            diesel::insert_into(chapters::table)
                .values(&new_chapter)
                .get_result::<Chapter>(self.conn)
                .await
                .map(chapter_output_from_record)
                .map_err(map_chapter_error)
        }
        .boxed()
    }

    fn update(
        &mut self,
        chapter_id: i32,
        command: UpdateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>> {
        async move {
            let update = UpdateChapter {
                title: command.title,
                order: command.order,
            };

            diesel::update(chapters::table.find(chapter_id))
                .set(&update)
                .get_result::<Chapter>(self.conn)
                .await
                .map(chapter_output_from_record)
                .map_err(map_chapter_error)
        }
        .boxed()
    }

    fn delete(&mut self, chapter_id: i32) -> BoxFuture<'_, Result<bool, ChapterError>> {
        async move {
            diesel::delete(chapters::table.find(chapter_id))
                .execute(self.conn)
                .await
                .map(|count| count > 0)
                .map_err(map_chapter_error)
        }
        .boxed()
    }
}

fn map_chapter_error(error: diesel::result::Error) -> ChapterError {
    match error {
        diesel::result::Error::NotFound => ChapterError::NotFound,
        other => ChapterError::Database(other.to_string()),
    }
}
