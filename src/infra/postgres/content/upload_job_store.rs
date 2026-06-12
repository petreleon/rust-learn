use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentProcessingJobStore;
use crate::application::content::process_upload_job::{ProcessUploadJobError, ProcessableContent};
use crate::db::schema::{chapters, contents, upload_jobs};
use crate::models::content::Content;
use crate::models::upload_job::NewUploadJob;

pub struct PostgresContentUploadJobStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresContentUploadJobStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ContentProcessingJobStore for PostgresContentUploadJobStore<'_> {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>> {
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

    fn content_for_processing(
        &mut self,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<ProcessableContent, ProcessUploadJobError>> {
        async move {
            contents::table
                .filter(contents::id.eq(content_id))
                .filter(contents::chapter_id.eq(chapter_id))
                .first::<Content>(self.conn)
                .await
                .map(|content| ProcessableContent {
                    content_type: content.content_type,
                    object_key: content.data,
                })
                .map_err(map_content_error)
        }
        .boxed()
    }

    fn enqueue_processing_job(
        &mut self,
        bucket: &'static str,
        object_key: String,
        user_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>> {
        async move {
            let new_job = NewUploadJob {
                bucket,
                object: &object_key,
                user_id: Some(user_id),
            };

            diesel::insert_into(upload_jobs::table)
                .values(&new_job)
                .execute(self.conn)
                .await
                .map(|_| ())
                .map_err(|error| ProcessUploadJobError::JobQueueFailed {
                    object_key,
                    message: error.to_string(),
                })
        }
        .boxed()
    }
}

fn map_chapter_error(error: diesel::result::Error) -> ProcessUploadJobError {
    match error {
        diesel::result::Error::NotFound => ProcessUploadJobError::ChapterNotFound,
        other => ProcessUploadJobError::ChapterLookupFailed(other.to_string()),
    }
}

fn map_content_error(error: diesel::result::Error) -> ProcessUploadJobError {
    match error {
        diesel::result::Error::NotFound => ProcessUploadJobError::ContentNotFound,
        other => ProcessUploadJobError::ContentLookupFailed(other.to_string()),
    }
}
