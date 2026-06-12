use actix_web::{web, HttpResponse, Responder};

use crate::application::content::request_media_url::{
    request_media_url as request_media_url_for_content, ContentMediaUrlError,
    RequestMediaUrlCommand,
};
use crate::db::DbPool;
use crate::http::content::dto::MediaUrlResponse;
use crate::infra::object_storage::content::media_url_provider::S3ContentMediaUrlProvider;
use crate::infra::postgres::content::media_object_store::PostgresContentMediaStore;
use crate::utils::s3_utils::S3State;

pub(super) async fn get_media_url(
    path: web::Path<(i32, i32, i32)>,
    pool: web::Data<DbPool>,
    s3: Option<web::Data<S3State>>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentMediaStore::new(&mut conn);
    let mut media_provider = match s3 {
        Some(s3) => S3ContentMediaUrlProvider::new(s3.get_ref().clone()),
        None => S3ContentMediaUrlProvider::from_env(),
    };

    match request_media_url_for_content(
        &mut store,
        &mut media_provider,
        RequestMediaUrlCommand {
            course_id,
            chapter_id,
            content_id,
        },
    )
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
