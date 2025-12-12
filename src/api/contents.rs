use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use diesel::{QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use crate::db::DbPool;
use crate::models::content::{Content, NewContent, UpdateContent};
use crate::db::schema::contents;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::config::constants::permissions::Permissions;
use crate::utils::minio_utils::MinioState;
use crate::models::upload_job::NewUploadJob;
use crate::db::schema::upload_jobs;
use crate::utils::jwt_utils::decode_jwt;


// #[get("/chapters/{id}/contents")]
async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (_course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .load::<Content>(&mut conn)
        .await;

    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            eprintln!("DB error listing contents: {}", e);
            HttpResponse::InternalServerError().body("Failed to list contents")
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateContentRequest {
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

async fn create_content(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
    req: web::Json<CreateContentRequest>,
) -> impl Responder {
    let (_course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
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
        .get_result::<Content>(&mut conn)
        .await;

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
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(contents::table.find(content_id))
        .set(&*req)
        .get_result::<Content>(&mut conn)
        .await;

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
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(contents::table.find(content_id))
        .execute(&mut conn)
        .await;

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


async fn process_content(
    req: actix_web::HttpRequest,
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (_course_id, _chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    // 1. Fetch Content to get the object key
    let content = match contents::table.find(content_id).first::<Content>(&mut conn).await {
        Ok(c) => c,
        Err(diesel::result::Error::NotFound) => return HttpResponse::NotFound().body("Content not found"),
        Err(e) => {
            eprintln!("DB error fetching content: {}", e);
            return HttpResponse::InternalServerError().body("Failed to fetch content");
        }
    };

    // 2. Validate it has data (object key)
    let object_key = match content.data {
        Some(d) if !d.is_empty() => d,
        _ => return HttpResponse::BadRequest().body("Content has no data/object key to process"),
    };

    // 3. Identify User (Optional, for notifications)
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or("");
    let token = if auth_header.starts_with("Bearer ") { &auth_header[7..] } else { "" };
    let user_id = decode_jwt(token).ok().map(|d| d.claims.user_id);

    // 4. Enqueue Job
    let new_job = NewUploadJob {
        bucket: "course-materials",
        object: &object_key,
        user_id,
    };

    let result = diesel::insert_into(upload_jobs::table)
        .values(&new_job)
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => HttpResponse::Accepted().body("Video processing queued"),
        Err(e) => {
            eprintln!("DB error queuing job: {}", e);
            HttpResponse::InternalServerError().body("Failed to queue processing job")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents")
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
        web::resource("/{course_id}/chapters/{chapter_id}/contents/upload_url")
            .route(web::post().to(get_upload_url)
                .wrap(CoursePermissionMiddleware::new(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "course_id".to_string()
                ))
            )
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}")
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
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/process")
            .route(web::post().to(process_content)
                .wrap(CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "course_id".to_string()
                ))
            )
    );
}
