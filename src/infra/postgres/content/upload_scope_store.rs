use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentUploadScopeStore;
use crate::application::content::request_upload_url::ContentUploadUrlError;
use crate::db::schema::chapters;

pub struct PostgresContentUploadScopeStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresContentUploadScopeStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ContentUploadScopeStore for PostgresContentUploadScopeStore<'_> {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ContentUploadUrlError>> {
        async move {
            chapters::table
                .filter(chapters::id.eq(chapter_id))
                .filter(chapters::course_id.eq(course_id))
                .select(chapters::id)
                .first::<i32>(self.conn)
                .await
                .map(|_| ())
                .map_err(map_scope_error)
        }
        .boxed()
    }
}

fn map_scope_error(error: diesel::result::Error) -> ContentUploadUrlError {
    match error {
        diesel::result::Error::NotFound => ContentUploadUrlError::ChapterNotFound,
        other => ContentUploadUrlError::Database(other.to_string()),
    }
}
