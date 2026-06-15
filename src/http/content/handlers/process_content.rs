use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::content::process_upload_job::{
    ContentProcessingUseCase, ProcessUploadJobCommand,
};
use crate::http::content::errors::process_upload_job_error;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;

pub(in crate::http::content) async fn process_content(
    user: AuthUserId,
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    processing_use_case: web::Data<Arc<dyn ContentProcessingUseCase>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let user_id = user.into_inner();

    processing_use_case
        .process_upload_job(ProcessUploadJobCommand {
            course_id,
            chapter_id,
            content_id,
            user_id,
        })
        .await
        .map(|_| ("Video processing queued", StatusCode::ACCEPTED))
        .map_err(|error| process_upload_job_error(course_id, chapter_id, content_id, error))
}
