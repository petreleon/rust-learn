use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_content_item::{ContentItemError, ContentItemUseCases};
use crate::application::content::request_upload_url::{
    ContentUploadUrlError, ContentUploadUrlUseCase,
};
use crate::http::content::dto::{
    ContentItemResponse, RequestUploadUrlRequest, UpdateContentItemRequest, UploadUrlResponse,
};

use super::content_item_error_log;

pub(in crate::http::content) async fn get_upload_url(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    upload_url_use_case: web::Data<Arc<dyn ContentUploadUrlUseCase>>,
    req: web::Json<RequestUploadUrlRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let request = req.into_inner();
    let object_path_for_log = format!(
        "courses/{}/chapters/{}/{}",
        course_id, chapter_id, request.filename
    );
    let command = request.into_command(course_id, chapter_id);

    match upload_url_use_case.request_upload_url(command).await {
        Ok(output) => HttpResponse::Ok().json(UploadUrlResponse::from(output)),
        Err(ContentUploadUrlError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentUploadUrlError::MissingContentType) => {
            HttpResponse::BadRequest().body("content_type is required")
        }
        Err(ContentUploadUrlError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
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
        Err(ContentUploadUrlError::StorageClientInitFailed(message)) => {
            log::error!(
                "event=content_upload_url_failed reason=s3_client_init course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to init storage client")
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

pub(in crate::http::content) async fn update_content(
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
    req: web::Json<UpdateContentItemRequest>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let command = req.into_inner().into();

    match content_item_use_cases
        .update_content_item(course_id, chapter_id, content_id, command)
        .await
    {
        Ok(content) => HttpResponse::Ok().json(ContentItemResponse::from(content)),
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentItemError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(ContentItemError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
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

pub(in crate::http::content) async fn delete_content(
    path: web::Path<(i32, i32, i32)>,
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();

    match content_item_use_cases
        .delete_content_item(course_id, chapter_id, content_id)
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
        Err(ContentItemError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
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
