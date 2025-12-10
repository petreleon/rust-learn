use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::content::{Content, NewContent, UpdateContent};
use crate::db::schema::contents;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::config::constants::permissions::Permissions;
use crate::utils::minio_utils::MinioState;

// #[get("/chapters/{id}/contents")]
async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (_course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .load::<Content>(&mut conn);

    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            eprintln!("DB error listing contents: {}", e);
            HttpResponse::InternalServerError().body("Failed to list contents")
        }
    }
}

async fn create_content(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
    req: web::Json<NewContent>,
) -> impl Responder {
    let (_course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let new_content = NewContent {
        chapter_id,
        order: req.order,
        content_type: req.content_type.clone(),
        data: req.data.clone(),
    };

    let result = diesel::insert_into(contents::table)
        .values(&new_content)
        .get_result::<Content>(&mut conn);

    match result {
        Ok(content) => HttpResponse::Created().json(content),
        Err(e) => {
            eprintln!("DB error creating content: {}", e);
            HttpResponse::InternalServerError().body("Failed to create content")
        }
    }
}

// Upload endpoint that returns a presigned URL for the client to upload file
#[derive(serde::Deserialize)]
struct UploadRequest {
    filename: String,
    content_type: String, // e.g. video/mp4, application/pdf
}

async fn get_upload_url(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    req: web::Json<UploadRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    
    // Construct object path: courses/{course_id}/chapters/{chapter_id}/{filename}
    let object_path = format!("courses/{}/chapters/{}/{}", course_id, chapter_id, req.filename);
    
    match MinioState::new_from_env().await {
        Ok(minio) => {
             // 1 hour expiry
             match minio.presign_put("course-materials", &object_path, 3600).await {
                 Ok(url) => HttpResponse::Ok().json(serde_json::json!({
                     "upload_url": url,
                     "object_key": object_path
                 })),
                 Err(e) => {
                     eprintln!("MinIO error: {}", e);
                     HttpResponse::InternalServerError().body("Failed to generate upload URL")
                 }
             }
        },
        Err(e) => {
            eprintln!("MinIO client init error: {}", e);
            HttpResponse::InternalServerError().body("Failed to init storage client")
        }
    }
}


async fn update_content(
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    pool: web::Data<DbPool>,
    req: web::Json<UpdateContent>,
) -> impl Responder {
    let (_course_id, _chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(contents::table.find(content_id))
        .set(&*req)
        .get_result::<Content>(&mut conn);

    match result {
        Ok(content) => HttpResponse::Ok().json(content),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Content not found"),
        Err(e) => {
            eprintln!("DB error updating content {}: {}", content_id, e);
            HttpResponse::InternalServerError().body("Failed to update content")
        }
    }
}

async fn delete_content(
    path: web::Path<(i32, i32, i32)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (_course_id, _chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(contents::table.find(content_id))
        .execute(&mut conn);

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Content deleted")
            } else {
                HttpResponse::NotFound().body("Content not found")
            }
        }
        Err(e) => {
            eprintln!("DB error deleting content {}: {}", content_id, e);
            HttpResponse::InternalServerError().body("Failed to delete content")
        }
    }
}

pub fn content_scope() -> actix_web::Scope {
    web::scope("")
        // All endpoints nested under /courses/{course_id}/chapters/{chapter_id} 
        // to simplify permission (course_id based).
        
        .service(
             web::resource("/courses/{course_id}/chapters/{chapter_id}/contents")
                .route(web::get().to(list_contents)
                    .wrap(CoursePermissionMiddleware::new(
                        Permissions::VIEW_COURSE.to_string(), 
                        ParamType::Path,
                        "course_id".to_string()
                    ))
                )
                .route(web::post().to(create_content)
                    .wrap(CoursePermissionMiddleware::new(
                         Permissions::MANAGE_COURSE_SETTINGS.to_string(), // Or CREATE_CONTENT
                         ParamType::Path,
                         "course_id".to_string()
                    ))
                )
        )
         .service(
             web::resource("/courses/{course_id}/chapters/{chapter_id}/contents/upload_url")
                .route(web::post().to(get_upload_url)
                    .wrap(CoursePermissionMiddleware::new(
                         Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                         ParamType::Path,
                         "course_id".to_string()
                    ))
                )
        )
        .service(
            web::resource("/courses/{course_id}/chapters/{chapter_id}/contents/{id}")
                .route(web::put().to(update_content)
                    .wrap(CoursePermissionMiddleware::new(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "course_id".to_string()
                    ))
                )
                .route(web::delete().to(delete_content)
                    .wrap(CoursePermissionMiddleware::new(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "course_id".to_string()
                    ))
                )
        )
}
