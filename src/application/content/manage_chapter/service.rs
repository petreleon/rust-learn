use futures::future::BoxFuture;

use crate::application::content::manage_chapter::{
    ChapterError, ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};

pub trait ChapterUseCases: Send + Sync {
    fn list_chapters(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ChapterOutput>, ChapterError>>;

    fn create_chapter(
        &self,
        command: CreateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>>;

    fn update_chapter(
        &self,
        chapter_id: i32,
        command: UpdateChapterCommand,
    ) -> BoxFuture<'_, Result<ChapterOutput, ChapterError>>;

    fn delete_chapter(&self, chapter_id: i32) -> BoxFuture<'_, Result<bool, ChapterError>>;
}
