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
