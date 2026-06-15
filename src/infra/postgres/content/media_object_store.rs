use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentMediaStore;
use crate::application::content::request_media_url::ContentMediaUrlError;
use crate::infra::postgres::schema::{chapters, contents};

pub struct PostgresContentMediaStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresContentMediaStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ContentMediaStore for PostgresContentMediaStore<'_> {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ContentMediaUrlError>> {
        async move {
            chapters::table
                .filter(chapters::id.eq(chapter_id))
                .filter(chapters::course_id.eq(course_id))
                .select(chapters::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_chapter_error)
        }
        .boxed()
    }

    fn content_object_key(
        &mut self,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<Option<String>, ContentMediaUrlError>> {
        async move {
            contents::table
                .filter(contents::id.eq(content_id))
                .filter(contents::chapter_id.eq(chapter_id))
                .select(contents::data)
                .first::<Option<String>>(self.conn)
                .await
                .map_err(map_content_error)
        }
        .boxed()
    }
}

fn map_chapter_error(error: diesel::result::Error) -> ContentMediaUrlError {
    match error {
        diesel::result::Error::NotFound => ContentMediaUrlError::ChapterNotFound,
        other => ContentMediaUrlError::ChapterLookupFailed(other.to_string()),
    }
}

fn map_content_error(error: diesel::result::Error) -> ContentMediaUrlError {
    match error {
        diesel::result::Error::NotFound => ContentMediaUrlError::ContentNotFound,
        other => ContentMediaUrlError::ContentLookupFailed(other.to_string()),
    }
}
