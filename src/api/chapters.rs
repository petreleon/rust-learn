use crate::db::schema::chapters;
use crate::db::DbPool;
use crate::models::chapter::{Chapter, NewChapter, UpdateChapter};
use actix_web::{web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Deserialize;

mod routes;

pub use routes::config;

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub new_order: i32,
}

// #[get("/courses/{id}/chapters")]
async fn list_chapters(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let course_id_val = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = chapters::table
        .filter(chapters::course_id.eq(course_id_val))
        .order(chapters::order.asc())
        .load::<Chapter>(&mut conn)
        .await;

    match result {
        Ok(chap_list) => HttpResponse::Ok().json(chap_list),
        Err(e) => {
            log::error!(
                "event=chapter_list_failed course_id={} error={}",
                course_id_val,
                e
            );
            HttpResponse::InternalServerError().body("Failed to load chapters")
        }
    }
}

#[derive(Deserialize)]
pub struct CreateChapterRequest {
    pub title: String,
    pub order: i32,
}

async fn create_chapter(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    req: web::Json<CreateChapterRequest>,
) -> impl Responder {
    let course_id_val = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let new_chapter = NewChapter {
        course_id: course_id_val,
        title: req.title.clone(),
        order: req.order,
    };

    let result = diesel::insert_into(chapters::table)
        .values(&new_chapter)
        .get_result::<Chapter>(&mut conn)
        .await;

    match result {
        Ok(chapter) => HttpResponse::Created().json(chapter),
        Err(e) => {
            log::error!(
                "event=chapter_create_failed course_id={} error={}",
                course_id_val,
                e
            );
            HttpResponse::InternalServerError().body("Failed to create chapter")
        }
    }
}

async fn update_chapter(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    req: web::Json<UpdateChapter>,
) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(chapters::table.find(chapter_id))
        .set(&*req)
        .get_result::<Chapter>(&mut conn)
        .await;

    match result {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Chapter not found"),
        Err(e) => {
            log::error!(
                "event=chapter_update_failed chapter_id={} error={}",
                chapter_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to update chapter")
        }
    }
}

async fn delete_chapter(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(chapters::table.find(chapter_id))
        .execute(&mut conn)
        .await;

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Chapter deleted")
            } else {
                HttpResponse::NotFound().body("Chapter not found")
            }
        }
        Err(e) => {
            log::error!(
                "event=chapter_delete_failed chapter_id={} error={}",
                chapter_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to delete chapter")
        }
    }
}
