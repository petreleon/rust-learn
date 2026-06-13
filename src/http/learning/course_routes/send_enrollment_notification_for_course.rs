async fn send_enrollment_notification_for_course(
    conn: &mut AsyncPgConnection,
    notifications: &NotificationsState,
    target_user_id: i32,
    course_id: i32,
) {
    let course_title = courses::table
        .find(course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await
        .unwrap_or_else(|_| format!("course #{}", course_id));

    if let Err(err) = notifications
        .send_enrollment_notification(target_user_id, course_id, course_title)
        .await
    {
        log::warn!(
            "event=notification_send_failed kind=enrollment course_id={} target_user_id={} error={:?}",
            course_id,
            target_user_id,
            err
        );
    }
}

async fn list_learner_course_catalog(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<LearnerCourseCatalogParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let catalog_query = LearnerCourseCatalogQuery::new(
        query.search.clone(),
        query.organization_id,
        query.lifecycle_status.clone(),
        query.enrollment_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_learner_course_catalog(&mut conn, requester.user_id, catalog_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

async fn list_teacher_course_dashboard(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<TeacherCourseDashboardParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let dashboard_query = TeacherCourseDashboardQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.limit,
        query.offset,
    );

    match discover_teacher_course_dashboard(&mut conn, requester.user_id, dashboard_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

async fn get_teacher_course_workspace_route(
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

    match get_teacher_course_workspace(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

async fn get_teacher_course_enrollment_workspace_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<TeacherCourseEnrollmentParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let enrollment_query =
        TeacherCourseEnrollmentQuery::new(query.status.clone(), query.limit, query.offset);
    match get_teacher_course_enrollment_workspace(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        enrollment_query,
    )
    .await
    {
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

async fn get_teacher_course_students_route(
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

    match get_teacher_course_students(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}
