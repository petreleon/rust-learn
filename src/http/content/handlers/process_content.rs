use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::content::process_upload_job::{
    ContentProcessingUseCase, ProcessUploadJobCommand, ProcessUploadJobError,
};
use crate::http::extractors::auth_user::AuthUserId;

pub(in crate::http::content) async fn process_content(
    user: AuthUserId,
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    processing_use_case: web::Data<Arc<dyn ContentProcessingUseCase>>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let user_id = user.into_inner();

    match processing_use_case
        .process_upload_job(ProcessUploadJobCommand {
            course_id,
            chapter_id,
            content_id,
            user_id,
        })
        .await
    {
        Ok(_) => HttpResponse::Accepted().body("Video processing queued"),
        Err(ProcessUploadJobError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ProcessUploadJobError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(ProcessUploadJobError::NonVideoContent) => {
            HttpResponse::BadRequest().body("Only video content can be processed")
        }
        Err(ProcessUploadJobError::MissingObjectKey) => {
            HttpResponse::BadRequest().body("Content has no data/object key to process")
        }
        Err(ProcessUploadJobError::InvalidObjectKey) => {
            HttpResponse::BadRequest().body("Content data must be a course upload object key")
        }
        Err(ProcessUploadJobError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(ProcessUploadJobError::ChapterLookupFailed(message)) => {
            log::error!(
                "event=content_process_failed reason=chapter_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch chapter")
        }
        Err(ProcessUploadJobError::ContentLookupFailed(message)) => {
            log::error!(
                "event=content_process_failed reason=content_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch content")
        }
        Err(ProcessUploadJobError::JobQueueFailed {
            object_key,
            message,
        }) => {
            log::error!(
                "event=content_process_failed reason=upload_job_insert course_id={} chapter_id={} content_id={} object={} error={}",
                course_id,
                chapter_id,
                content_id,
                object_key,
                message
            );
            HttpResponse::InternalServerError().body("Failed to queue processing job")
        }
    }
}
