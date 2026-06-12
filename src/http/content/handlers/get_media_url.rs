use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::content::request_media_url::{
    ContentMediaUrlError, ContentMediaUrlUseCase, RequestMediaUrlCommand,
};
use crate::http::content::dto::MediaUrlResponse;

pub(in crate::http::content) async fn get_media_url(
    path: web::Path<(i32, i32, i32)>,
    media_url_use_case: web::Data<Arc<dyn ContentMediaUrlUseCase>>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();

    match media_url_use_case
        .request_media_url(RequestMediaUrlCommand {
            course_id,
            chapter_id,
            content_id,
        })
        .await
    {
        Ok(output) => HttpResponse::Ok().json(MediaUrlResponse::from(output)),
        Err(ContentMediaUrlError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentMediaUrlError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(ContentMediaUrlError::MissingObjectKey) => {
            HttpResponse::BadRequest().body("Content has no stored data/object key")
        }
        Err(ContentMediaUrlError::InvalidObjectKey) => {
            HttpResponse::BadRequest().body("Content data is not an upload object key")
        }
        Err(ContentMediaUrlError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(ContentMediaUrlError::ChapterLookupFailed(message)) => {
            log::error!(
                "event=content_media_lookup_failed reason=chapter_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch chapter")
        }
        Err(ContentMediaUrlError::ContentLookupFailed(message)) => {
            log::error!(
                "event=content_media_lookup_failed course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch content")
        }
        Err(ContentMediaUrlError::StorageClientInitFailed(message)) => {
            log::error!(
                "event=content_media_url_failed reason=s3_client_init course_id={} content_id={} error={}",
                course_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to init storage client")
        }
        Err(ContentMediaUrlError::PresignFailed {
            object_key,
            message,
        }) => {
            log::error!(
                "event=content_media_url_failed reason=presign_get course_id={} chapter_id={} content_id={} object={} error={}",
                course_id, chapter_id, content_id, object_key, message
            );
            HttpResponse::InternalServerError().body("Failed to generate media URL")
        }
    }
}
