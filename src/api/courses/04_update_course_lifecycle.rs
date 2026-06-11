async fn update_course_lifecycle(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseLifecycleUpdateRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match crate::services::course_service::update_course_lifecycle(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => lifecycle_error_response(error),
    }
}

async fn request_course_join(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match request_course_join_for_actor(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(join_request) => HttpResponse::Created().json(join_request),
        Err(error) => course_enrollment_error_response(error),
    }
}

async fn decide_course_join_request(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> impl Responder {
    let reviewer = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, request_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let status_before_decision = course_join_requests::table
        .find(request_id)
        .select(course_join_requests::status)
        .first::<String>(&mut conn)
        .await
        .optional()
        .ok()
        .flatten();

    match decide_course_join_request_for_actor(
        &mut conn,
        reviewer.user_id,
        course_id,
        request_id,
        body.into_inner(),
    )
    .await
    {
        Ok(join_request) => {
            if join_request.status == COURSE_JOIN_STATUS_APPROVED
                && status_before_decision.as_deref() != Some(COURSE_JOIN_STATUS_APPROVED)
            {
                if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                    send_enrollment_notification_for_course(
                        &mut conn,
                        notifications,
                        join_request.requester_user_id,
                        join_request.course_id,
                    )
                    .await;
                }
            }

            HttpResponse::Ok().json(join_request)
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

async fn remove_course_enrollment(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let actor = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, target_user_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match remove_course_enrollment_for_actor(&mut conn, actor.user_id, course_id, target_user_id)
        .await
    {
        Ok(removal) => HttpResponse::Ok().json(removal),
        Err(error) => course_enrollment_error_response(error),
    }
}

async fn delete_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(courses::table.find(course_id))
        .execute(&mut conn)
        .await;

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Course deleted")
            } else {
                HttpResponse::NotFound().body("Course not found")
            }
        }
        Err(e) => {
            log::error!(
                "event=course_delete_failed course_id={} error={}",
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to delete course")
        }
    }
}
