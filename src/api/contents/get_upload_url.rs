use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_content_item::{self, ContentItemError};
use crate::application::content::request_upload_url::{
    request_upload_url as request_upload_url_for_content, ContentUploadUrlError,
};
use crate::db::DbPool;
use crate::http::content::dto::{
    ContentItemResponse, RequestUploadUrlRequest, UpdateContentItemRequest, UploadUrlResponse,
};
use crate::infra::object_storage::content::upload_url_provider::S3ContentUploadUrlProvider;
use crate::infra::postgres::content::content_item_store::PostgresContentItemStore;
use crate::infra::postgres::content::upload_scope_store::PostgresContentUploadScopeStore;
use crate::utils::s3_utils::S3State;

use super::content_item_error_log;

pub(super) async fn get_upload_url(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
    s3: Option<web::Data<S3State>>,
    req: web::Json<RequestUploadUrlRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut scope_store = PostgresContentUploadScopeStore::new(&mut conn);
    let request = req.into_inner();
    let object_path_for_log = format!(
        "courses/{}/chapters/{}/{}",
        course_id, chapter_id, request.filename
    );

    let mut upload_provider = match s3 {
        Some(s3) => S3ContentUploadUrlProvider::new(s3.get_ref().clone()),
        None => match S3ContentUploadUrlProvider::new_from_env().await {
            Ok(provider) => provider,
            Err(e) => {
                log::error!(
                    "event=content_upload_url_failed reason=s3_client_init course_id={} chapter_id={} error={}",
                    course_id,
                    chapter_id,
                    e
                );
                return HttpResponse::InternalServerError().body("Failed to init storage client");
            }
        },
    };
    let command = request.into_command(course_id, chapter_id);

    match request_upload_url_for_content(&mut scope_store, &mut upload_provider, command).await {
        Ok(output) => HttpResponse::Ok().json(UploadUrlResponse::from(output)),
        Err(ContentUploadUrlError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentUploadUrlError::MissingContentType) => {
            HttpResponse::BadRequest().body("content_type is required")
        }
        Err(ContentUploadUrlError::Database(message)) => {
            log::error!(
                "event=content_upload_url_failed reason=chapter_lookup course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch chapter")
        }
        Err(ContentUploadUrlError::BucketPrepareFailed(message)) => {
            log::error!(
                "event=content_upload_url_failed reason=s3_bucket_init course_id={} chapter_id={} bucket=course-materials error={}",
                course_id,
                chapter_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to prepare upload bucket")
        }
        Err(ContentUploadUrlError::PresignFailed(message)) => {
            log::error!(
                "event=content_upload_url_failed reason=presign_put course_id={} chapter_id={} bucket=course-materials object={} error={}",
                course_id,
                chapter_id,
                object_path_for_log,
                message
            );
            HttpResponse::InternalServerError().body("Failed to generate upload URL")
        }
    }
}

pub(super) async fn update_content(
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    pool: web::Data<DbPool>,
    req: web::Json<UpdateContentItemRequest>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentItemStore::new(&mut conn);
    let command = req.into_inner().into();

    match manage_content_item::update_content_item(
        &mut store, course_id, chapter_id, content_id, command,
    )
    .await
    {
        Ok(content) => HttpResponse::Ok().json(ContentItemResponse::from(content)),
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentItemError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(e) => {
            log::error!(
                "event=content_update_failed content_id={} error={}",
                content_id,
                content_item_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to update content")
        }
    }
}

pub(super) async fn delete_content(
    path: web::Path<(i32, i32, i32)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentItemStore::new(&mut conn);

    match manage_content_item::delete_content_item(&mut store, course_id, chapter_id, content_id)
        .await
    {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().body("Content deleted")
            } else {
                HttpResponse::NotFound().body("Content not found")
            }
        }
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentItemError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(e) => {
            log::error!(
                "event=content_delete_failed content_id={} error={}",
                content_id,
                content_item_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to delete content")
        }
    }
}
