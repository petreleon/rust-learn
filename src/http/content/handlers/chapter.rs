use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_chapter::{self, ChapterError};
use crate::db::DbPool;
use crate::http::content::dto::{ChapterResponse, CreateChapterRequest, UpdateChapterRequest};
use crate::infra::postgres::content::chapter_store::PostgresChapterStore;

use super::chapter_error_log;

// #[get("/courses/{id}/chapters")]
pub(in crate::http::content) async fn list_chapters(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let course_id_val = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresChapterStore::new(&mut conn);

    match manage_chapter::list_chapters(&mut store, course_id_val).await {
        Ok(chap_list) => HttpResponse::Ok().json(
            chap_list
                .into_iter()
                .map(ChapterResponse::from)
                .collect::<Vec<_>>(),
        ),
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
    pool: web::Data<DbPool>,
    req: web::Json<CreateChapterRequest>,
) -> impl Responder {
    let course_id_val = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresChapterStore::new(&mut conn);
    let command = req.into_inner().into_command(course_id_val);

    match manage_chapter::create_chapter(&mut store, command).await {
        Ok(chapter) => HttpResponse::Created().json(ChapterResponse::from(chapter)),
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
    pool: web::Data<DbPool>,
    req: web::Json<UpdateChapterRequest>,
) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresChapterStore::new(&mut conn);
    let command = req.into_inner().into();

    match manage_chapter::update_chapter(&mut store, chapter_id, command).await {
        Ok(chapter) => HttpResponse::Ok().json(ChapterResponse::from(chapter)),
        Err(ChapterError::NotFound) => HttpResponse::NotFound().body("Chapter not found"),
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
    pool: web::Data<DbPool>,
) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresChapterStore::new(&mut conn);

    match manage_chapter::delete_chapter(&mut store, chapter_id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().body("Chapter deleted")
            } else {
                HttpResponse::NotFound().body("Chapter not found")
            }
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
