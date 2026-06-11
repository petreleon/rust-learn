async fn get_course_organizations(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    use crate::models::organization::Organization;

    let result = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .inner_join(crate::db::schema::organizations::table)
        .select(crate::db::schema::organizations::all_columns)
        .load::<Organization>(&mut conn)
        .await;

    match result {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(e) => {
            log::error!(
                "event=course_organizations_fetch_failed course_id={} error={}",
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to fetch course organizations")
        }
    }
}

async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (course_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    // Identify Requester from JWT
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    // Permission Check: Handled by Middleware
    // Middleware "MANAGE_COURSE_ENROLLMENTS" required.

    // Perform Assignment with Hierarchy Check
    match assign_role_to_user_in_course(&mut conn, requester_id, target_user_id, course_id, role_name).await {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "course",
                        Some(course_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=course course_id={} target_user_id={} error={:?}",
                        course_id,
                        target_user_id,
                        err
                    );
                }

            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(diesel::result::Error::RollbackTransaction) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(diesel::result::Error::NotFound) => HttpResponse::BadRequest().body("Role or User not found"),
        Err(e) => {
            log::error!(
                "event=course_role_assign_failed requester_user_id={} target_user_id={} course_id={} role={} error={}",
                requester_id,
                target_user_id,
                course_id,
                role_name,
                e
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}

#[derive(serde::Deserialize)]
struct SaveProgressRequest {
    content_id: i32,
}

async fn save_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<SaveProgressRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match save_learner_progress(&mut conn, user_id, course_id, body.content_id).await {
        Ok(progress) => HttpResponse::Ok().json(progress),
        Err(e) => {
            log::error!(
                "event=learner_progress_save_failed user_id={} course_id={} content_id={} error={:?}",
                user_id, course_id, body.content_id, e
            );
            HttpResponse::InternalServerError().body("Failed to save progress")
        }
    }
}

async fn get_learner_progress_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_progress(&mut conn, user_id, course_id).await {
        Ok(progress) => HttpResponse::Ok().json(progress),
        Err(e) => {
            log::error!(
                "event=learner_progress_fetch_failed user_id={} course_id={} error={:?}",
                user_id,
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to fetch progress")
        }
    }
}
