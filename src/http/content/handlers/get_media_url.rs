use std::sync::Arc;

use actix_web::web;

use crate::application::content::request_media_url::{
    ContentMediaUrlUseCase, RequestMediaUrlCommand,
};
use crate::http::content::dto::MediaUrlResponse;
use crate::http::content::errors::media_url_error;
use crate::http::errors::ApiError;

pub(in crate::http::content) async fn get_media_url(
    path: web::Path<(i32, i32, i32)>,
    media_url_use_case: web::Data<Arc<dyn ContentMediaUrlUseCase>>,
) -> Result<web::Json<MediaUrlResponse>, ApiError> {
    let (course_id, chapter_id, content_id) = path.into_inner();

    media_url_use_case
        .request_media_url(RequestMediaUrlCommand {
            course_id,
            chapter_id,
            content_id,
        })
        .await
        .map(MediaUrlResponse::from)
        .map(web::Json)
        .map_err(|error| media_url_error(course_id, chapter_id, content_id, error))
}
