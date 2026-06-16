use crate::application::content::manage_chapter::{chapter_output, ChapterFact, ChapterOutput};
use crate::application::content::manage_content_item::{
    content_item_output, ContentItemFact, ContentItemOutput,
};
use crate::infra::postgres::models::chapter::Chapter;
use crate::infra::postgres::models::content::Content;

pub(super) fn chapter_output_from_record(chapter: Chapter) -> ChapterOutput {
    chapter_output(ChapterFact {
        id: chapter.id,
        course_id: chapter.course_id,
        title: chapter.title,
        order: chapter.order,
    })
}

pub(super) fn content_item_output_from_record(content: Content) -> ContentItemOutput {
    content_item_output(ContentItemFact {
        id: content.id,
        chapter_id: content.chapter_id,
        order: content.order,
        content_type: content.content_type,
        data: content.data,
        publication_status: content.publication_status,
    })
}
