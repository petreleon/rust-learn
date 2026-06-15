use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::content::manage_content_item::ContentItemUseCases;
use crate::application::content::request_upload_url::ContentUploadUrlUseCase;
use crate::http::content::dto::{
    ContentItemResponse, RequestUploadUrlRequest, UpdateContentItemRequest, UploadUrlResponse,
};
use crate::http::content::errors::{content_item_error, content_not_found, upload_url_error};
use crate::http::errors::ApiError;

pub(in crate::http::content) async fn get_upload_url(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    upload_url_use_case: web::Data<Arc<dyn ContentUploadUrlUseCase>>,
    req: web::Json<RequestUploadUrlRequest>,
) -> Result<web::Json<UploadUrlResponse>, ApiError> {
    let (course_id, chapter_id) = path.into_inner();
    let request = req.into_inner();
    let object_path_for_log = format!(
        "courses/{}/chapters/{}/{}",
        course_id, chapter_id, request.filename
    );
    let command = request.into_command(course_id, chapter_id);

    upload_url_use_case
        .request_upload_url(command)
        .await
        .map(UploadUrlResponse::from)
        .map(web::Json)
        .map_err(|error| upload_url_error(course_id, chapter_id, &object_path_for_log, error))
}

pub(in crate::http::content) async fn update_content(
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
    req: web::Json<UpdateContentItemRequest>,
) -> Result<web::Json<ContentItemResponse>, ApiError> {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let command = req.into_inner().into();

    content_item_use_cases
        .update_content_item(course_id, chapter_id, content_id, command)
        .await
        .map(ContentItemResponse::from)
        .map(web::Json)
        .map_err(|error| {
            content_item_error(
                "content_update_failed",
                format!("content_id={}", content_id),
                "Failed to update content",
                error,
            )
        })
}

pub(in crate::http::content) async fn delete_content(
    path: web::Path<(i32, i32, i32)>,
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let (course_id, chapter_id, content_id) = path.into_inner();

    let deleted = content_item_use_cases
        .delete_content_item(course_id, chapter_id, content_id)
        .await
        .map_err(|error| {
            content_item_error(
                "content_delete_failed",
                format!("content_id={}", content_id),
                "Failed to delete content",
                error,
            )
        })?;
    if deleted {
        Ok(("Content deleted", StatusCode::OK))
    } else {
        Err(content_not_found())
    }
}
