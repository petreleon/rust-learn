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
