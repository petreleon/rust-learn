use std::sync::Arc;

use actix_web::web;

use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryQuery, ContentProcessingHistoryUseCase,
};
use crate::http::content::dto::ContentProcessingHistoryResponse;
use crate::http::content::errors::processing_history_error;
use crate::http::errors::ApiError;

pub(in crate::http::content) async fn get_processing_history(
    path: web::Path<(i32, i32, i32)>,
    use_case: web::Data<Arc<dyn ContentProcessingHistoryUseCase>>,
) -> Result<web::Json<ContentProcessingHistoryResponse>, ApiError> {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let query = ContentProcessingHistoryQuery {
        course_id,
        chapter_id,
        content_id,
    };

    use_case
        .inspect_content_processing_history(query)
        .await
        .map(ContentProcessingHistoryResponse::from)
        .map(web::Json)
        .map_err(|error| processing_history_error(course_id, chapter_id, content_id, error))
}
