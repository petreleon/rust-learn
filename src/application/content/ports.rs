use futures::future::BoxFuture;

use crate::application::content::manage_chapter::{
    ChapterError, ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};

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
