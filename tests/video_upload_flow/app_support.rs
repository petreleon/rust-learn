use crate::support::{DbPool, S3State};
use actix_web::{web, App};
use rust_learn::application::content::manage_chapter::ChapterUseCases;
use rust_learn::application::content::manage_content_item::ContentItemUseCases;
use rust_learn::application::content::process_upload_job::ContentProcessingUseCase;
use rust_learn::application::content::request_media_url::ContentMediaUrlUseCase;
use rust_learn::application::content::request_upload_url::ContentUploadUrlUseCase;
use rust_learn::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use rust_learn::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use rust_learn::infra::postgres::content::media_url_use_case::PostgresContentMediaUrlUseCase;
use rust_learn::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use rust_learn::infra::postgres::content::upload_url_use_case::PostgresContentUploadUrlUseCase;
use std::sync::Arc;

fn chapter_use_cases_data(pool: &DbPool) -> web::Data<Arc<dyn ChapterUseCases>> {
    web::Data::new(Arc::new(PostgresChapterUseCases::new(pool.clone())))
}

fn content_item_use_cases_data(pool: &DbPool) -> web::Data<Arc<dyn ContentItemUseCases>> {
    web::Data::new(Arc::new(PostgresContentItemUseCases::new(pool.clone())))
}

fn upload_url_use_case_data(
    pool: &DbPool,
    s3: &S3State,
) -> web::Data<Arc<dyn ContentUploadUrlUseCase>> {
    web::Data::new(Arc::new(PostgresContentUploadUrlUseCase::new(
        pool.clone(),
        s3.clone(),
    )))
}

fn media_url_use_case_data(
    pool: &DbPool,
    s3: &S3State,
) -> web::Data<Arc<dyn ContentMediaUrlUseCase>> {
    web::Data::new(Arc::new(PostgresContentMediaUrlUseCase::new(
        pool.clone(),
        s3.clone(),
    )))
}

fn content_processing_use_case_data(pool: &DbPool) -> web::Data<Arc<dyn ContentProcessingUseCase>> {
    web::Data::new(Arc::new(PostgresContentProcessingUseCase::new(
        pool.clone(),
    )))
}

pub(crate) fn content_test_app(
    pool: &DbPool,
    s3: &S3State,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .configure(|cfg| rust_learn::bootstrap::configure_access_control_check_app_data(cfg, pool))
        .app_data(chapter_use_cases_data(pool))
        .app_data(content_item_use_cases_data(pool))
        .app_data(upload_url_use_case_data(pool, s3))
        .app_data(media_url_use_case_data(pool, s3))
        .app_data(content_processing_use_case_data(pool))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(rust_learn::http::learning::course_scope())
}
