use crate::application::content::manage_chapter::ChapterError;
use crate::application::content::manage_content_item::ContentItemError;

mod chapter;
mod create_content;
mod get_media_url;
mod get_upload_url;
mod process_content;

pub(super) use chapter::{create_chapter, delete_chapter, list_chapters, update_chapter};
pub(super) use create_content::{create_content, list_contents};
pub(super) use get_media_url::get_media_url;
pub(super) use get_upload_url::{delete_content, get_upload_url, update_content};
pub(super) use process_content::process_content;

fn chapter_error_log(error: &ChapterError) -> String {
    match error {
        ChapterError::NotFound => "not_found".to_string(),
        ChapterError::Connection(message) => message.clone(),
        ChapterError::Database(message) => message.clone(),
    }
}

fn content_item_error_log(error: &ContentItemError) -> String {
    match error {
        ContentItemError::ChapterNotFound => "chapter_not_found".to_string(),
        ContentItemError::ContentNotFound => "content_not_found".to_string(),
        ContentItemError::Connection(message) => message.clone(),
        ContentItemError::Database(message) => message.clone(),
    }
}
