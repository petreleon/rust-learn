use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryError, ContentProcessingHistoryOutput, ContentProcessingHistoryQuery,
    ContentProcessingJobOutput,
};
use crate::application::content::manage_content_item::ContentItemError;
use crate::application::content::ports::ContentProcessingHistoryStore;
use crate::infra::postgres::content::content_item_scope::{
    ensure_chapter_belongs_to_course, ensure_content_belongs_to_chapter,
};
use crate::infra::postgres::models::upload_job::UploadJob;
use crate::infra::postgres::schema::{contents, upload_jobs};

pub struct PostgresContentProcessingHistoryStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresContentProcessingHistoryStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl ContentProcessingHistoryStore for PostgresContentProcessingHistoryStore<'_> {
    fn processing_history(
        &mut self,
        query: ContentProcessingHistoryQuery,
    ) -> BoxFuture<'_, Result<ContentProcessingHistoryOutput, ContentProcessingHistoryError>> {
        async move {
            ensure_chapter_belongs_to_course(self.conn, query.course_id, query.chapter_id)
                .await
                .map_err(map_content_item_error)?;
            ensure_content_belongs_to_chapter(self.conn, query.chapter_id, query.content_id)
                .await
                .map_err(map_content_item_error)?;

            let object_key = load_content_object_key(self.conn, query.content_id).await?;
            let jobs = match object_key.as_deref() {
                Some(key) => load_processing_jobs(self.conn, key).await?,
                None => Vec::new(),
            };

            Ok(ContentProcessingHistoryOutput {
                content_id: query.content_id,
                object_key,
                jobs,
            })
        }
        .boxed()
    }
}

async fn load_content_object_key(
    conn: &mut AsyncPgConnection,
    content_id: i32,
) -> Result<Option<String>, ContentProcessingHistoryError> {
    contents::table
        .find(content_id)
        .select(contents::data)
        .first::<Option<String>>(conn)
        .await
        .map(normalize_object_key)
        .map_err(map_database_error)
}

async fn load_processing_jobs(
    conn: &mut AsyncPgConnection,
    object_key: &str,
) -> Result<Vec<ContentProcessingJobOutput>, ContentProcessingHistoryError> {
    upload_jobs::table
        .filter(upload_jobs::object.eq(object_key))
        .order(upload_jobs::created_at.desc())
        .then_order_by(upload_jobs::id.desc())
        .load::<UploadJob>(conn)
        .await
        .map(|jobs| jobs.into_iter().map(processing_job_output).collect())
        .map_err(map_database_error)
}

fn processing_job_output(job: UploadJob) -> ContentProcessingJobOutput {
    ContentProcessingJobOutput {
        id: job.id,
        status: job.status,
        attempts: job.attempts,
        last_error: job.last_error,
        created_at: job.created_at,
        updated_at: job.updated_at,
    }
}

fn normalize_object_key(value: Option<String>) -> Option<String> {
    value
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty())
}

fn map_content_item_error(error: ContentItemError) -> ContentProcessingHistoryError {
    match error {
        ContentItemError::ChapterNotFound => ContentProcessingHistoryError::ChapterNotFound,
        ContentItemError::ContentNotFound => ContentProcessingHistoryError::ContentNotFound,
        ContentItemError::Connection(message) => ContentProcessingHistoryError::Connection(message),
        ContentItemError::Database(message) => ContentProcessingHistoryError::Database(message),
    }
}

fn map_database_error(error: diesel::result::Error) -> ContentProcessingHistoryError {
    match error {
        diesel::result::Error::NotFound => ContentProcessingHistoryError::ContentNotFound,
        other => ContentProcessingHistoryError::Database(other.to_string()),
    }
}
