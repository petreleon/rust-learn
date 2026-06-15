use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::content::manage_chapter::ChapterUseCases;
use crate::http::content::dto::{ChapterResponse, CreateChapterRequest, UpdateChapterRequest};
use crate::http::content::errors::{chapter_error, chapter_not_found};
use crate::http::errors::ApiError;

// #[get("/courses/{id}/chapters")]
pub(in crate::http::content) async fn list_chapters(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
) -> Result<web::Json<Vec<ChapterResponse>>, ApiError> {
    let course_id_val = path.into_inner();

    chapter_use_cases
        .list_chapters(course_id_val)
        .await
        .map(|chapters| chapters.into_iter().map(ChapterResponse::from).collect())
        .map(web::Json)
        .map_err(|error| {
            chapter_error(
                "chapter_list_failed",
                format!("course_id={}", course_id_val),
                "Failed to load chapters",
                error,
            )
        })
}

pub(in crate::http::content) async fn create_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
    req: web::Json<CreateChapterRequest>,
) -> Result<(web::Json<ChapterResponse>, StatusCode), ApiError> {
    let course_id_val = path.into_inner();
    let command = req.into_inner().into_command(course_id_val);

    chapter_use_cases
        .create_chapter(command)
        .await
        .map(ChapterResponse::from)
        .map(web::Json)
        .map(|body| (body, StatusCode::CREATED))
        .map_err(|error| {
            chapter_error(
                "chapter_create_failed",
                format!("course_id={}", course_id_val),
                "Failed to create chapter",
                error,
            )
        })
}

pub(in crate::http::content) async fn update_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
    req: web::Json<UpdateChapterRequest>,
) -> Result<web::Json<ChapterResponse>, ApiError> {
    let chapter_id = path.into_inner();
    let command = req.into_inner().into();

    chapter_use_cases
        .update_chapter(chapter_id, command)
        .await
        .map(ChapterResponse::from)
        .map(web::Json)
        .map_err(|error| {
            chapter_error(
                "chapter_update_failed",
                format!("chapter_id={}", chapter_id),
                "Failed to update chapter",
                error,
            )
        })
}

pub(in crate::http::content) async fn delete_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let chapter_id = path.into_inner();

    let deleted = chapter_use_cases
        .delete_chapter(chapter_id)
        .await
        .map_err(|error| {
            chapter_error(
                "chapter_delete_failed",
                format!("chapter_id={}", chapter_id),
                "Failed to delete chapter",
                error,
            )
        })?;
    if deleted {
        Ok(("Chapter deleted", StatusCode::OK))
    } else {
        Err(chapter_not_found())
    }
}
