use crate::config::constants::permissions::Permissions;
use crate::db::schema::chapters;
use crate::db::schema::contents;
use crate::db::schema::upload_jobs;
use crate::db::schema::user_role_course;
use crate::db::DbPool;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::content::{Content, NewContent, UpdateContent};
use crate::models::param_type::ParamType;
use crate::models::upload_job::NewUploadJob;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;
use crate::utils::s3_utils::S3State;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

async fn ensure_chapter_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    chapter_id: i32,
) -> Result<(), HttpResponse> {
    match chapters::table
        .filter(chapters::id.eq(chapter_id))
        .filter(chapters::course_id.eq(course_id))
        .select(chapters::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("Chapter not found"))
        }
        Err(e) => {
            log::error!(
                "event=content_chapter_scope_check_failed course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                e
            );
            Err(HttpResponse::InternalServerError().body("Failed to fetch chapter"))
        }
    }
}

async fn ensure_content_belongs_to_chapter(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_id: i32,
) -> Result<(), HttpResponse> {
    match contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .select(contents::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("Content not found"))
        }
        Err(e) => {
            log::error!(
                "event=content_scope_check_failed chapter_id={} content_id={} error={}",
                chapter_id,
                content_id,
                e
            );
            Err(HttpResponse::InternalServerError().body("Failed to fetch content"))
        }
    }
}

// #[get("/chapters/{id}/contents")]
async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }

    let result = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .load::<Content>(&mut conn)
        .await;

    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            log::error!(
                "event=content_list_failed chapter_id={} error={}",
                chapter_id,
                e
            );
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
    notifications: Option<web::Data<NotificationsState>>,
    req: web::Json<CreateContentRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }

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
        Ok(content) => {
            if let Some(notifications) = notifications {
                let recipient_ids = user_role_course::table
                    .filter(user_role_course::course_id.eq(course_id))
                    .select(user_role_course::user_id)
                    .distinct()
                    .load::<Option<i32>>(&mut conn)
                    .await
                    .unwrap_or_else(|err| {
                        log::warn!(
                            "event=notification_recipient_query_failed kind=content_published course_id={} content_id={} error={:?}",
                            course_id,
                            content.id,
                            err
                        );
                        Vec::new()
                    });

                for recipient_id in recipient_ids.into_iter().flatten() {
                    if let Err(err) = notifications
                        .send_content_published_notification(
                            recipient_id,
                            course_id,
                            content.id,
                            &content.content_type,
                        )
                        .await
                    {
                        log::warn!(
                            "event=notification_send_failed kind=content_published course_id={} content_id={} target_user_id={} error={:?}",
                            course_id,
                            content.id,
                            recipient_id,
                            err
                        );
                    }
                }
            }

            HttpResponse::Created().json(content)
        }
        Err(e) => {
            log::error!(
                "event=content_create_failed course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                e
            );
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
    pool: web::Data<DbPool>,
    s3: Option<web::Data<S3State>>,
    req: web::Json<UploadRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }

    if req.content_type.trim().is_empty() {
        return HttpResponse::BadRequest().body("content_type is required");
    }

    // Construct object path: courses/{course_id}/chapters/{chapter_id}/{filename}
    let object_path = format!(
        "courses/{}/chapters/{}/{}",
        course_id, chapter_id, req.filename
    );

    let s3 = match s3 {
        Some(s3) => s3,
        None => match S3State::new_from_env().await {
            Ok(s3) => web::Data::new(s3),
            Err(e) => {
                log::error!(
                    "event=content_upload_url_failed reason=s3_client_init course_id={} chapter_id={} error={}",
                    course_id,
                    chapter_id,
                    e
                );
                return HttpResponse::InternalServerError().body("Failed to init storage client");
            }
        },
    };

    if let Err(e) = s3.ensure_bucket("course-materials").await {
        log::error!(
            "event=content_upload_url_failed reason=s3_bucket_init course_id={} chapter_id={} bucket=course-materials error={}",
            course_id,
            chapter_id,
            e
        );
        return HttpResponse::InternalServerError().body("Failed to prepare upload bucket");
    }

    // 1 hour expiry
    match s3
        .presign_external_put("course-materials", &object_path, 3600)
        .await
    {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({
            "upload_url": url,
            "object_key": object_path
        })),
        Err(e) => {
            log::error!(
                "event=content_upload_url_failed reason=presign_put course_id={} chapter_id={} bucket=course-materials object={} error={}",
                course_id,
                chapter_id,
                object_path,
                e
            );
            HttpResponse::InternalServerError().body("Failed to generate upload URL")
        }
    }
}

async fn update_content(
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    pool: web::Data<DbPool>,
    req: web::Json<UpdateContent>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }
    if let Err(response) =
        ensure_content_belongs_to_chapter(&mut conn, chapter_id, content_id).await
    {
        return response;
    }

    let result = diesel::update(contents::table.find(content_id))
        .set(&*req)
        .get_result::<Content>(&mut conn)
        .await;

    match result {
        Ok(content) => HttpResponse::Ok().json(content),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Content not found"),
        Err(e) => {
            log::error!(
                "event=content_update_failed content_id={} error={}",
                content_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to update content")
        }
    }
}

async fn delete_content(
    path: web::Path<(i32, i32, i32)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }
    if let Err(response) =
        ensure_content_belongs_to_chapter(&mut conn, chapter_id, content_id).await
    {
        return response;
    }

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
            log::error!(
                "event=content_delete_failed content_id={} error={}",
                content_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to delete content")
        }
    }
}

fn is_video_content_type(content_type: &str) -> bool {
    let normalized = content_type.trim().to_ascii_lowercase();
    normalized == "video" || normalized.starts_with("video/")
}

async fn process_content(
    req: HttpRequest,
    path: web::Path<(i32, i32, i32)>, // course_id, chapter_id, content_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let chapter_exists = chapters::table
        .filter(chapters::id.eq(chapter_id))
        .filter(chapters::course_id.eq(course_id))
        .select(chapters::id)
        .first::<i32>(&mut conn)
        .await;

    match chapter_exists {
        Ok(_) => {}
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("Chapter not found")
        }
        Err(e) => {
            log::error!(
                "event=content_process_failed reason=chapter_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                e
            );
            return HttpResponse::InternalServerError().body("Failed to fetch chapter");
        }
    }

    // 1. Fetch Content to get the object key
    let content = match contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .first::<Content>(&mut conn)
        .await
    {
        Ok(c) => c,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("Content not found")
        }
        Err(e) => {
            log::error!(
                "event=content_process_failed reason=content_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                e
            );
            return HttpResponse::InternalServerError().body("Failed to fetch content");
        }
    };

    if !is_video_content_type(&content.content_type) {
        return HttpResponse::BadRequest().body("Only video content can be processed");
    }

    // 2. Validate it has data (object key)
    let object_key = match content.data {
        Some(d) if !d.trim().is_empty() => d.trim().to_string(),
        _ => return HttpResponse::BadRequest().body("Content has no data/object key to process"),
    };
    let expected_prefix = format!("courses/{}/chapters/{}/", course_id, chapter_id);
    if !object_key.starts_with(&expected_prefix) {
        return HttpResponse::BadRequest().body("Content data must be a course upload object key");
    }

    // 3. Enqueue Job
    let new_job = NewUploadJob {
        bucket: "course-materials",
        object: &object_key,
        user_id: Some(user_id),
    };

    let result = diesel::insert_into(upload_jobs::table)
        .values(&new_job)
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => HttpResponse::Accepted().body("Video processing queued"),
        Err(e) => {
            log::error!(
                "event=content_process_failed reason=upload_job_insert course_id={} chapter_id={} content_id={} object={} error={}",
                course_id,
                chapter_id,
                content_id,
                object_key,
                e
            );
            HttpResponse::InternalServerError().body("Failed to queue processing job")
        }
    }
}

async fn get_media_url(
    path: web::Path<(i32, i32, i32)>,
    pool: web::Data<DbPool>,
    s3: Option<web::Data<S3State>>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await {
        return response;
    }

    let content = match contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .first::<Content>(&mut conn)
        .await
    {
        Ok(c) => c,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("Content not found")
        }
        Err(e) => {
            log::error!(
                "event=content_media_lookup_failed course_id={} chapter_id={} content_id={} error={}",
                course_id, chapter_id, content_id, e
            );
            return HttpResponse::InternalServerError().body("Failed to fetch content");
        }
    };

    let object_key = match content.data {
        Some(d) if !d.trim().is_empty() => d.trim().to_string(),
        _ => return HttpResponse::BadRequest().body("Content has no stored data/object key"),
    };

    let expected_prefix = format!("courses/{}/chapters/{}/", course_id, chapter_id);
    if !object_key.starts_with(&expected_prefix) {
        return HttpResponse::BadRequest().body("Content data is not an upload object key");
    }

    let s3 = match s3 {
        Some(s3) => s3,
        None => match S3State::new_from_env().await {
            Ok(s3) => web::Data::new(s3),
            Err(e) => {
                log::error!(
                    "event=content_media_url_failed reason=s3_client_init course_id={} content_id={} error={}",
                    course_id, content_id, e
                );
                return HttpResponse::InternalServerError().body("Failed to init storage client");
            }
        },
    };

    match s3.presign_external_get("course-materials", &object_key, 3600).await {
        Ok(url) => HttpResponse::Ok().json(serde_json::json!({ "url": url })),
        Err(e) => {
            log::error!(
                "event=content_media_url_failed reason=presign_get course_id={} chapter_id={} content_id={} object={} error={}",
                course_id, chapter_id, content_id, object_key, e
            );
            HttpResponse::InternalServerError().body("Failed to generate media URL")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents")
            .route(
                web::get()
                    .to(list_contents)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::VIEW_CONTENT.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            )
            .route(
                web::post()
                    .to(create_content)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::CREATE_CONTENT.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/upload_url").route(
            web::post()
                .to(get_upload_url)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::CREATE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}")
            .route(
                web::put()
                    .to(update_content)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::MODIFY_CONTENT.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            )
            .route(
                web::delete()
                    .to(delete_content)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::DELETE_CONTENT.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/process").route(
            web::post()
                .to(process_content)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::MODIFY_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/media").route(
            web::get()
                .to(get_media_url)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::VIEW_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    );
}
