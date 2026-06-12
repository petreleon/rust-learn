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
    let mut store = PostgresContentMediaStore::new(&mut conn);
    let mut media_provider = match s3 {
        Some(s3) => S3ContentMediaUrlProvider::new(s3.get_ref().clone()),
        None => S3ContentMediaUrlProvider::from_env(),
    };

    match request_media_url_for_content(
        &mut store,
        &mut media_provider,
        RequestMediaUrlCommand {
            course_id,
            chapter_id,
            content_id,
        },
    )
    .await
    {
        Ok(output) => HttpResponse::Ok().json(MediaUrlResponse::from(output)),
        Err(ContentMediaUrlError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentMediaUrlError::ContentNotFound) => {
            HttpResponse::NotFound().body("Content not found")
        }
        Err(ContentMediaUrlError::MissingObjectKey) => {
            HttpResponse::BadRequest().body("Content has no stored data/object key")
        }
        Err(ContentMediaUrlError::InvalidObjectKey) => {
            HttpResponse::BadRequest().body("Content data is not an upload object key")
        }
        Err(ContentMediaUrlError::ChapterLookupFailed(message)) => {
            log::error!(
                "event=content_media_lookup_failed reason=chapter_lookup course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch chapter")
        }
        Err(ContentMediaUrlError::ContentLookupFailed(message)) => {
            log::error!(
                "event=content_media_lookup_failed course_id={} chapter_id={} content_id={} error={}",
                course_id,
                chapter_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to fetch content")
        }
        Err(ContentMediaUrlError::StorageClientInitFailed(message)) => {
            log::error!(
                "event=content_media_url_failed reason=s3_client_init course_id={} content_id={} error={}",
                course_id,
                content_id,
                message
            );
            HttpResponse::InternalServerError().body("Failed to init storage client")
        }
        Err(ContentMediaUrlError::PresignFailed {
            object_key,
            message,
        }) => {
            log::error!(
                "event=content_media_url_failed reason=presign_get course_id={} chapter_id={} content_id={} object={} error={}",
                course_id, chapter_id, content_id, object_key, message
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
