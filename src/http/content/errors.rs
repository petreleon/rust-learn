use actix_web::http::StatusCode;

use crate::application::content::inspect_processing_history::ContentProcessingHistoryError;
use crate::application::content::manage_chapter::ChapterError;
use crate::application::content::manage_content_item::ContentItemError;
use crate::http::errors::ApiError;

mod transfer;

pub(super) use transfer::{media_url_error, process_upload_job_error, upload_url_error};

pub(super) fn chapter_error(
    event: &'static str,
    context: String,
    message: &'static str,
    error: ChapterError,
) -> ApiError {
    match error {
        ChapterError::NotFound => chapter_not_found(),
        ChapterError::Connection(_) => db_connection_failed(),
        ChapterError::Database(error) => logged_internal(event, &context, message, error),
    }
}

pub(super) fn content_item_error(
    event: &'static str,
    context: String,
    message: &'static str,
    error: ContentItemError,
) -> ApiError {
    match error {
        ContentItemError::ChapterNotFound => chapter_not_found(),
        ContentItemError::ContentNotFound => content_not_found(),
        ContentItemError::Connection(_) => db_connection_failed(),
        ContentItemError::Database(error) => logged_internal(event, &context, message, error),
    }
}

pub(super) fn processing_history_error(
    course_id: i32,
    chapter_id: i32,
    content_id: i32,
    error: ContentProcessingHistoryError,
) -> ApiError {
    match error {
        ContentProcessingHistoryError::ChapterNotFound => chapter_not_found(),
        ContentProcessingHistoryError::ContentNotFound => content_not_found(),
        ContentProcessingHistoryError::Connection(_) => db_connection_failed(),
        ContentProcessingHistoryError::Database(error) => logged_internal(
            "content_processing_history_failed",
            &format!(
                "course_id={} chapter_id={} content_id={}",
                course_id, chapter_id, content_id
            ),
            "Failed to inspect content processing history",
            error,
        ),
    }
}

pub(super) fn chapter_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "chapter_not_found",
        "Chapter not found",
    )
}

pub(super) fn content_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "content_not_found",
        "Content not found",
    )
}

pub(super) fn bad_request(message: &'static str) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, "invalid_content_request", message)
}

pub(super) fn db_connection_failed() -> ApiError {
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "db_connection_failed",
        "Failed to get DB connection",
    )
}

pub(super) fn logged_internal(
    event: &'static str,
    context: &str,
    message: &'static str,
    error: String,
) -> ApiError {
    log::error!("event={} {} error={}", event, context, error);
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "content_failed", message)
}

#[cfg(test)]
mod tests {
    use super::{chapter_not_found, process_upload_job_error};
    use crate::application::content::process_upload_job::ProcessUploadJobError;
    use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
    use serde_json::Value;

    #[actix_web::test]
    async fn chapter_not_found_uses_api_error_envelope() {
        let response = chapter_not_found().error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::NOT_FOUND);
        assert_eq!(body.value["error"]["code"], "chapter_not_found");
        assert_eq!(body.value["error"]["message"], "Chapter not found");
    }

    #[actix_web::test]
    async fn non_video_processing_error_uses_bad_request_envelope() {
        let response = process_upload_job_error(1, 2, 3, ProcessUploadJobError::NonVideoContent)
            .error_response();
        let body = parse_body(response).await;

        assert_eq!(body.status, StatusCode::BAD_REQUEST);
        assert_eq!(body.value["error"]["code"], "invalid_content_request");
        assert_eq!(
            body.value["error"]["message"],
            "Only video content can be processed"
        );
    }

    struct ParsedErrorBody {
        status: StatusCode,
        value: Value,
    }

    async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
        let status = response.status();
        let body = to_bytes(response.into_body()).await.unwrap();
        let value = serde_json::from_slice(&body).unwrap();
        ParsedErrorBody { status, value }
    }
}
