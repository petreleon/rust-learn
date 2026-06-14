use futures::future::BoxFuture;

use crate::application::content::manage_content_item::{
    ContentItemError, ContentItemOutput, CreateContentItemCommand, CreateContentItemOutput,
    UpdateContentItemCommand,
};

pub trait ContentItemUseCases: Send + Sync {
    fn list_content_items(
        &self,
        course_id: i32,
        chapter_id: i32,
    ) -> BoxFuture<'_, Result<Vec<ContentItemOutput>, ContentItemError>>;

    fn create_content_item(
        &self,
        course_id: i32,
        command: CreateContentItemCommand,
    ) -> BoxFuture<'_, Result<CreateContentItemOutput, ContentItemError>>;

    fn update_content_item(
        &self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
        command: UpdateContentItemCommand,
    ) -> BoxFuture<'_, Result<ContentItemOutput, ContentItemError>>;

    fn delete_content_item(
        &self,
        course_id: i32,
        chapter_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, ContentItemError>>;
}
