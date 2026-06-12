use std::sync::Arc;

use crate::application::content::manage_chapter::ChapterUseCases;
use crate::application::content::manage_content_item::ContentItemUseCases;
use crate::application::content::process_upload_job::ContentProcessingUseCase;
use crate::application::content::request_media_url::ContentMediaUrlUseCase;
use crate::application::content::request_upload_url::ContentUploadUrlUseCase;
use crate::db::DbPool;
use crate::utils::notifications::NotificationsState;
use crate::utils::s3_utils::S3State;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub s3: S3State,
    pub notifications: NotificationsState,
    pub chapter_use_cases: Arc<dyn ChapterUseCases>,
    pub content_item_use_cases: Arc<dyn ContentItemUseCases>,
    pub content_upload_url_use_case: Arc<dyn ContentUploadUrlUseCase>,
    pub content_media_url_use_case: Arc<dyn ContentMediaUrlUseCase>,
    pub content_processing_use_case: Arc<dyn ContentProcessingUseCase>,
}
