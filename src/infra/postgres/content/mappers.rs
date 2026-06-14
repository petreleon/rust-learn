use crate::application::content::manage_content_item::ContentItemOutput;
use crate::models::content::Content;

impl From<Content> for ContentItemOutput {
    fn from(content: Content) -> Self {
        Self {
            id: content.id,
            chapter_id: content.chapter_id,
            order: content.order,
            content_type: content.content_type,
            data: content.data,
        }
    }
}
