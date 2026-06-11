async fn list_assessment_attempts(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    use crate::db::schema::assessment_attempts;
    use crate::models::assessment::AssessmentAttempt;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (_course_id, assessment_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    match assessment_attempts::table
        .filter(assessment_attempts::assessment_id.eq(assessment_id))
        .filter(assessment_attempts::user_id.eq(user_id))
        .order(assessment_attempts::started_at.desc())
        .load::<AssessmentAttempt>(&mut conn)
        .await
    {
        Ok(attempts) => HttpResponse::Ok().json(attempts),
        Err(e) => {
            log::error!("event=assessment_attempts_failed error={}", e);
            HttpResponse::InternalServerError().body("Failed to load attempts")
        }
    }
}

pub fn course_scope() -> actix_web::Scope {
    web::scope("/courses")
        .configure(crate::api::chapters::config)
        .configure(crate::api::contents::config)
        .configure(crate::api::reward_candidates::configure_course_reward_candidate_routes)
        .service(web::resource("/catalog").route(web::get().to(list_learner_course_catalog)))
        .service(web::resource("/teaching").route(web::get().to(list_teacher_course_dashboard)))
        .service(
            web::resource("/teaching/{id}/enrollments")
                .route(web::get().to(get_teacher_course_enrollment_workspace_route)),
        )
        .service(
            web::resource("/teaching/{id}/students")
                .route(web::get().to(get_teacher_course_students_route)),
        )
        .service(
            web::resource("/teaching/{id}")
                .route(web::get().to(get_teacher_course_workspace_route)),
        )
        .service(
            web::resource("/catalog/{id}/learn")
                .route(web::get().to(get_learner_course_learning_route)),
        )
        .service(
            web::resource("/catalog/{id}").route(web::get().to(get_learner_course_catalog_detail)),
        )
        .service(
            web::resource("")
                .route(
                    web::get()
                        .to(list_courses)
                        .wrap(PlatformPermissionMiddleware::require(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::post().to(create_course)),
        )
        .service(
            web::resource("/{id}/organizations").route(
                web::get().to(get_course_organizations).wrap(
                    PlatformPermissionMiddleware::require(Permissions::VIEW_COURSE.to_string()),
                ),
            ),
        )
        .service(web::resource("/{id}/lifecycle").route(web::put().to(update_course_lifecycle)))
        .service(web::resource("/{id}/join-requests").route(web::post().to(request_course_join)))
        .service(
            web::resource("/{id}/join-requests/{request_id}/decision")
                .route(web::put().to(decide_course_join_request)),
        )
        .service(
            web::resource("/{id}/enrollments/{user_id}")
                .route(web::delete().to(remove_course_enrollment)),
        )
        .service(
            web::resource("/{id}/progress")
                .route(web::get().to(get_learner_progress_route))
                .route(web::post().to(save_learner_progress_route)),
        )
        .service(web::resource("/{id}/assessments").route(web::get().to(list_course_assessments)))
        .service(
            web::resource("/{id}/assessments/{assessment_id}/submit")
                .route(web::post().to(submit_assessment_attempt)),
        )
        .service(
            web::resource("/{id}/assessments/{assessment_id}/attempts")
                .route(web::get().to(list_assessment_attempts)),
        )
        .service(
            web::resource("/{id}")
                .route(
                    web::get()
                        .to(get_course)
                        .wrap(PlatformPermissionMiddleware::require(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::put().to(update_course))
                .route(
                    web::delete()
                        .to(delete_course)
                        .wrap(CoursePermissionMiddleware::require(
                            Permissions::DELETE_COURSE.to_string(),
                            ParamType::Path,
                            "id".to_string(),
                        )),
                ),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles").route(web::post().to(assign_role).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            )),
        )
}
