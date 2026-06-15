use crate::application::content::process_upload_job::ProcessUploadJobError;
use crate::application::content::request_media_url::ContentMediaUrlError;
use crate::application::content::request_upload_url::ContentUploadUrlError;
use crate::http::errors::ApiError;

pub(in crate::http::content) fn upload_url_error(
    course_id: i32,
    chapter_id: i32,
    object_path: &str,
    error: ContentUploadUrlError,
) -> ApiError {
    match error {
        ContentUploadUrlError::ChapterNotFound => super::chapter_not_found(),
        ContentUploadUrlError::MissingContentType => super::bad_request("content_type is required"),
        ContentUploadUrlError::Connection(_) => super::db_connection_failed(),
        ContentUploadUrlError::Database(error) => super::logged_internal(
            "content_upload_url_failed",
            &format!("reason=chapter_lookup course_id={course_id} chapter_id={chapter_id}"),
            "Failed to fetch chapter",
            error,
        ),
        ContentUploadUrlError::StorageClientInitFailed(error) => super::logged_internal(
            "content_upload_url_failed",
            &format!("reason=s3_client_init course_id={course_id} chapter_id={chapter_id}"),
            "Failed to init storage client",
            error,
        ),
        ContentUploadUrlError::BucketPrepareFailed(error) => super::logged_internal(
            "content_upload_url_failed",
            &format!("reason=s3_bucket_init course_id={course_id} chapter_id={chapter_id}"),
            "Failed to prepare upload bucket",
            error,
        ),
        ContentUploadUrlError::PresignFailed(error) => super::logged_internal(
            "content_upload_url_failed",
            &format!(
                "reason=presign_put course_id={course_id} chapter_id={chapter_id} object={object_path}"
            ),
            "Failed to generate upload URL",
            error,
        ),
    }
}

pub(in crate::http::content) fn media_url_error(
    course_id: i32,
    chapter_id: i32,
    content_id: i32,
    error: ContentMediaUrlError,
) -> ApiError {
    match error {
        ContentMediaUrlError::ChapterNotFound => super::chapter_not_found(),
        ContentMediaUrlError::ContentNotFound => super::content_not_found(),
        ContentMediaUrlError::MissingObjectKey => {
            super::bad_request("Content has no stored data/object key")
        }
        ContentMediaUrlError::InvalidObjectKey => {
            super::bad_request("Content data is not an upload object key")
        }
        ContentMediaUrlError::Connection(_) => super::db_connection_failed(),
        ContentMediaUrlError::ChapterLookupFailed(error) => super::logged_internal(
            "content_media_lookup_failed",
            &format!(
                "reason=chapter_lookup course_id={course_id} chapter_id={chapter_id} content_id={content_id}"
            ),
            "Failed to fetch chapter",
            error,
        ),
        ContentMediaUrlError::ContentLookupFailed(error) => super::logged_internal(
            "content_media_lookup_failed",
            &format!("course_id={course_id} chapter_id={chapter_id} content_id={content_id}"),
            "Failed to fetch content",
            error,
        ),
        ContentMediaUrlError::StorageClientInitFailed(error) => super::logged_internal(
            "content_media_url_failed",
            &format!("reason=s3_client_init course_id={course_id} content_id={content_id}"),
            "Failed to init storage client",
            error,
        ),
        ContentMediaUrlError::PresignFailed {
            object_key,
            message,
        } => super::logged_internal(
            "content_media_url_failed",
            &format!(
                "reason=presign_get course_id={course_id} chapter_id={chapter_id} content_id={content_id} object={object_key}"
            ),
            "Failed to generate media URL",
            message,
        ),
    }
}

pub(in crate::http::content) fn process_upload_job_error(
    course_id: i32,
    chapter_id: i32,
    content_id: i32,
    error: ProcessUploadJobError,
) -> ApiError {
    match error {
        ProcessUploadJobError::ChapterNotFound => super::chapter_not_found(),
        ProcessUploadJobError::ContentNotFound => super::content_not_found(),
        ProcessUploadJobError::NonVideoContent => {
            super::bad_request("Only video content can be processed")
        }
        ProcessUploadJobError::MissingObjectKey => {
            super::bad_request("Content has no data/object key to process")
        }
        ProcessUploadJobError::InvalidObjectKey => {
            super::bad_request("Content data must be a course upload object key")
        }
        ProcessUploadJobError::Connection(_) => super::db_connection_failed(),
        ProcessUploadJobError::ChapterLookupFailed(error) => super::logged_internal(
            "content_process_failed",
            &format!(
                "reason=chapter_lookup course_id={course_id} chapter_id={chapter_id} content_id={content_id}"
            ),
            "Failed to fetch chapter",
            error,
        ),
        ProcessUploadJobError::ContentLookupFailed(error) => super::logged_internal(
            "content_process_failed",
            &format!(
                "reason=content_lookup course_id={course_id} chapter_id={chapter_id} content_id={content_id}"
            ),
            "Failed to fetch content",
            error,
        ),
        ProcessUploadJobError::JobQueueFailed {
            object_key,
            message,
        } => super::logged_internal(
            "content_process_failed",
            &format!(
                "reason=upload_job_insert course_id={course_id} chapter_id={chapter_id} content_id={content_id} object={object_key}"
            ),
            "Failed to queue processing job",
            message,
        ),
    }
}
