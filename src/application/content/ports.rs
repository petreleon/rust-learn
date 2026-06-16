use futures::future::BoxFuture;

use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryError, ContentProcessingHistoryOutput, ContentProcessingHistoryQuery,
};
use crate::application::content::manage_chapter::{
    ChapterError, ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};
use crate::application::content::manage_content_item::{
    ContentItemError, ContentItemOutput, CreateContentItemCommand, UpdateContentItemCommand,
};
use crate::application::content::process_upload_job::{ProcessUploadJobError, ProcessableContent};
use crate::application::content::request_media_url::ContentMediaUrlError;
use crate::application::content::request_upload_url::ContentUploadUrlError;

pub trait ChapterStore {
    fn list_by_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ChapterOutput>, ChapterError>>;

    fn create(
        &mut self,
        command: CreateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>>;

    fn update(
        &mut self,
        chapter_id: i32,
        command: UpdateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>>;

    fn delete(&mut self, chapter_id: i32) -> BoxFuture<'_, Result<bool, ChapterError>>;
}

pub trait ContentItemStore {
    fn list_by_chapter(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ContentItemOutput>, ContentItemError>>;

    fn create(
        &mut self,
        course_id: i32,
        command: CreateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>>;

    fn list_course_content_recipients(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, ContentItemError>>;

    fn update(
        &mut self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
        command: UpdateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>>;

    fn delete(
        &mut self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, ContentItemError>>;
}

pub trait ContentUploadScopeStore {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ContentUploadUrlError>>;
}

pub trait ContentUploadUrlProvider {
    fn prepare_upload_url(
        &mut self,
        bucket: &'static str,
        object_key: String,
        expires_seconds: u64,
    ) -> BoxFuture<'_, Result<String, ContentUploadUrlError>>;
}

pub trait ContentMediaStore {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ContentMediaUrlError>>;

    fn content_object_key(
        &mut self,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<Option<String>, ContentMediaUrlError>>;
}

pub trait ContentMediaUrlProvider {
    fn media_url(
        &mut self,
        bucket: &'static str,
        object_key: String,
        expires_seconds: u64,
    ) -> BoxFuture<'_, Result<String, ContentMediaUrlError>>;
}

pub trait ContentProcessingJobStore {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>>;

    fn content_for_processing(
        &mut self,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<ProcessableContent, ProcessUploadJobError>>;

    fn enqueue_processing_job(
        &mut self,
        bucket: &'static str,
        object_key: String,
        user_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>>;
}

pub trait ContentProcessingHistoryStore {
    fn processing_history(
        &mut self,
        query: ContentProcessingHistoryQuery,
    ) -> BoxFuture<'_, Result<ContentProcessingHistoryOutput, ContentProcessingHistoryError>>;
}
