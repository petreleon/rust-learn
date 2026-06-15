use std::sync::Arc;

use actix_web::web;

use crate::application::content::manage_chapter::ChapterUseCases;
use crate::application::content::manage_content_item::ContentItemUseCases;
use crate::application::content::process_upload_job::ContentProcessingUseCase;
use crate::application::content::request_media_url::ContentMediaUrlUseCase;
use crate::application::content::request_upload_url::ContentUploadUrlUseCase;
use crate::infra::object_storage::S3State;
use crate::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use crate::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use crate::infra::postgres::content::media_url_use_case::PostgresContentMediaUrlUseCase;
use crate::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use crate::infra::postgres::content::upload_url_use_case::PostgresContentUploadUrlUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct ContentUseCases {
    pub chapter: Arc<dyn ChapterUseCases>,
    pub item: Arc<dyn ContentItemUseCases>,
    pub media_url: Arc<dyn ContentMediaUrlUseCase>,
    pub processing: Arc<dyn ContentProcessingUseCase>,
    pub upload_url: Arc<dyn ContentUploadUrlUseCase>,
}

pub fn build_content_use_cases(pool: &DbPool, s3: &S3State) -> ContentUseCases {
    ContentUseCases {
        chapter: Arc::new(PostgresChapterUseCases::new(pool.clone())),
        item: Arc::new(PostgresContentItemUseCases::new(pool.clone())),
        media_url: Arc::new(PostgresContentMediaUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
        processing: Arc::new(PostgresContentProcessingUseCase::new(pool.clone())),
        upload_url: Arc::new(PostgresContentUploadUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
    }
}

pub fn configure_content_app_data(cfg: &mut web::ServiceConfig, use_cases: &ContentUseCases) {
    cfg.app_data(web::Data::new(use_cases.chapter.clone()))
        .app_data(web::Data::new(use_cases.item.clone()))
        .app_data(web::Data::new(use_cases.media_url.clone()))
        .app_data(web::Data::new(use_cases.processing.clone()))
        .app_data(web::Data::new(use_cases.upload_url.clone()));
}
