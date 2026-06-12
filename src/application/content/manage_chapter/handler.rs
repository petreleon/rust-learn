use crate::application::content::manage_chapter::{
    ChapterError, ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};
use crate::application::content::ports::ChapterStore;

pub async fn list_chapters(
    store: &mut impl ChapterStore,
    course_id: i32,
) -> Result<Vec<ChapterOutput>, ChapterError> {
    store.list_by_course(course_id).await
}

pub async fn create_chapter(
    store: &mut impl ChapterStore,
    command: CreateChapterCommand,
) -> Result<ChapterOutput, ChapterError> {
    store.create(command).await
}

pub async fn update_chapter(
    store: &mut impl ChapterStore,
    chapter_id: i32,
    command: UpdateChapterCommand,
) -> Result<ChapterOutput, ChapterError> {
    store.update(chapter_id, command).await
}

pub async fn delete_chapter(
    store: &mut impl ChapterStore,
    chapter_id: i32,
) -> Result<bool, ChapterError> {
    store.delete(chapter_id).await
}
