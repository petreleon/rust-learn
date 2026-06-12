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
    req: web::Json<UpdateContentItemRequest>,
) -> impl Responder {
    let (course_id, chapter_id, content_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentItemStore::new(&mut conn);
    let command = req.into_inner().into();

    match manage_content_item::update_content_item(
        &mut store, course_id, chapter_id, content_id, command,
    )
    .await
    {
        Ok(content) => HttpResponse::Ok().json(ContentItemResponse::from(content)),
        Err(ContentItemError::ChapterNotFound) => HttpResponse::NotFound().body("Chapter not found"),
        Err(ContentItemError::ContentNotFound) => HttpResponse::NotFound().body("Content not found"),
        Err(e) => {
            log::error!(
                "event=content_update_failed content_id={} error={}",
                content_id,
                content_item_error_log(&e)
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
    let mut store = PostgresContentItemStore::new(&mut conn);

    match manage_content_item::delete_content_item(&mut store, course_id, chapter_id, content_id)
        .await
    {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().body("Content deleted")
            } else {
                HttpResponse::NotFound().body("Content not found")
            }
        }
        Err(ContentItemError::ChapterNotFound) => HttpResponse::NotFound().body("Chapter not found"),
        Err(ContentItemError::ContentNotFound) => HttpResponse::NotFound().body("Content not found"),
        Err(e) => {
            log::error!(
                "event=content_delete_failed content_id={} error={}",
                content_id,
                content_item_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to delete content")
        }
    }
}

fn is_video_content_type(content_type: &str) -> bool {
    let normalized = content_type.trim().to_ascii_lowercase();
    normalized == "video" || normalized.starts_with("video/")
}
