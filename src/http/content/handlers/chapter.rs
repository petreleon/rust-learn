use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_chapter::{ChapterError, ChapterUseCases};
use crate::http::content::dto::{ChapterResponse, CreateChapterRequest, UpdateChapterRequest};

use super::chapter_error_log;

// #[get("/courses/{id}/chapters")]
pub(in crate::http::content) async fn list_chapters(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
) -> impl Responder {
    let course_id_val = path.into_inner();

    match chapter_use_cases.list_chapters(course_id_val).await {
        Ok(chap_list) => HttpResponse::Ok().json(
            chap_list
                .into_iter()
                .map(ChapterResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(ChapterError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=chapter_list_failed course_id={} error={}",
                course_id_val,
                chapter_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to load chapters")
        }
    }
}

pub(in crate::http::content) async fn create_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
    req: web::Json<CreateChapterRequest>,
) -> impl Responder {
    let course_id_val = path.into_inner();
    let command = req.into_inner().into_command(course_id_val);

    match chapter_use_cases.create_chapter(command).await {
        Ok(chapter) => HttpResponse::Created().json(ChapterResponse::from(chapter)),
        Err(ChapterError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=chapter_create_failed course_id={} error={}",
                course_id_val,
                chapter_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to create chapter")
        }
    }
}

pub(in crate::http::content) async fn update_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
    req: web::Json<UpdateChapterRequest>,
) -> impl Responder {
    let chapter_id = path.into_inner();
    let command = req.into_inner().into();

    match chapter_use_cases.update_chapter(chapter_id, command).await {
        Ok(chapter) => HttpResponse::Ok().json(ChapterResponse::from(chapter)),
        Err(ChapterError::NotFound) => HttpResponse::NotFound().body("Chapter not found"),
        Err(ChapterError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=chapter_update_failed chapter_id={} error={}",
                chapter_id,
                chapter_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to update chapter")
        }
    }
}

pub(in crate::http::content) async fn delete_chapter(
    path: web::Path<i32>,
    chapter_use_cases: web::Data<Arc<dyn ChapterUseCases>>,
) -> impl Responder {
    let chapter_id = path.into_inner();

    match chapter_use_cases.delete_chapter(chapter_id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().body("Chapter deleted")
            } else {
                HttpResponse::NotFound().body("Chapter not found")
            }
        }
        Err(ChapterError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=chapter_delete_failed chapter_id={} error={}",
                chapter_id,
                chapter_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to delete chapter")
        }
    }
}
