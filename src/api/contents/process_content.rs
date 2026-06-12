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
    let mut store = PostgresContentUploadJobStore::new(&mut conn);

    match process_upload_job_for_content(
        &mut store,
        ProcessUploadJobCommand {
            course_id,
            chapter_id,
            content_id,
            user_id,
        },
    )
    .await
    {
        Ok(_) => HttpResponse::Accepted().body("Video processing queued"),
        Err(ProcessUploadJobError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ProcessUploadJobError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(ProcessUploadJobError::NonVideoContent) => {
            HttpResponse::BadRequest().body("Only video content can be processed")
        }
        Err(ProcessUploadJobError::MissingObjectKey) => {
            HttpResponse::BadRequest().body("Content has no data/object key to process")
        }
        Err(ProcessUploadJobError::InvalidObjectKey) => {
            HttpResponse::BadRequest().body("Content data must be a course upload object key")
        }
        Err(ProcessUploadJobError::ChapterLookupFailed(message)) => {
            log::error!(
                "event=content_process_failed reason=chapter_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch chapter")
        }
        Err(ProcessUploadJobError::ContentLookupFailed(message)) => {
            log::error!(
                "event=content_process_failed reason=content_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch content")
        }
        Err(ProcessUploadJobError::JobQueueFailed {
            object_key,
            message,
        }) => {
            log::error!(
                "event=content_process_failed reason=upload_job_insert course_id={} chapter_id={} content_id={} object={} error={}",
                course_id,
                chapter_id,
                content_id,
                object_key,
                message
            );
            HttpResponse::InternalServerError().body("Failed to queue processing job")
        }
    }
}
