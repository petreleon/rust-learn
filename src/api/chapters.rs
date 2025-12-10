use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::chapter::{Chapter, NewChapter, UpdateChapter};
use crate::db::schema::chapters;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::config::constants::permissions::Permissions;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub new_order: i32,
}

// #[get("/courses/{id}/chapters")]
async fn list_chapters(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let course_id_val = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = chapters::table
        .filter(chapters::course_id.eq(course_id_val))
        .order(chapters::order.asc())
        .load::<Chapter>(&mut conn);

    match result {
        Ok(chap_list) => HttpResponse::Ok().json(chap_list),
        Err(e) => {
            eprintln!("DB error listing chapters: {}", e);
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
    let mut conn = match pool.get() {
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
        .get_result::<Chapter>(&mut conn);

    match result {
        Ok(chapter) => HttpResponse::Created().json(chapter),
        Err(e) => {
            eprintln!("DB error creating chapter: {}", e);
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
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(chapters::table.find(chapter_id))
        .set(&*req)
        .get_result::<Chapter>(&mut conn);

    match result {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Chapter not found"),
        Err(e) => {
            eprintln!("DB error updating chapter {}: {}", chapter_id, e);
            HttpResponse::InternalServerError().body("Failed to update chapter")
        }
    }
}

async fn delete_chapter(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(chapters::table.find(chapter_id))
        .execute(&mut conn);

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Chapter deleted")
            } else {
                HttpResponse::NotFound().body("Chapter not found")
            }
        }
        Err(e) => {
            eprintln!("DB error deleting chapter {}: {}", chapter_id, e);
            HttpResponse::InternalServerError().body("Failed to delete chapter")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}/chapters")
            .route(web::get().to(list_chapters)
                .wrap(CoursePermissionMiddleware::new(
                    Permissions::VIEW_COURSE.to_string(), // Student can view
                    ParamType::Path,
                    "id".to_string()
                ))
            )
            .route(web::post().to(create_chapter)
                .wrap(CoursePermissionMiddleware::new(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(), // Teacher+
                        ParamType::Path,
                        "id".to_string()
                ))
            )
    )
    .service(
        web::resource("/{course_id}/chapters/{id}")
            .route(web::put().to(update_chapter)
                .wrap(CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "course_id".to_string()
                ))
            )
            .route(web::delete().to(delete_chapter)
                .wrap(CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "course_id".to_string()
                ))
            )
    );
}
