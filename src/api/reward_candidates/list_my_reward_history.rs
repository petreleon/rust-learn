async fn list_course_reward_candidates(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::list_course_reward_candidates(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        query.into_inner(),
    )
    .await
    {
        Ok(candidates) => HttpResponse::Ok().json(candidates),
        Err(error) => reward_candidate_error_response(error),
    }
}

async fn list_platform_reward_candidates(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<PlatformRewardCandidatesRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::list_platform_reward_candidates(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => reward_candidate_error_response(error),
    }
}

async fn list_reward_candidate_audit(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match reward_candidate_service::list_reward_candidate_audit(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(error) => reward_candidate_error_response(error),
    }
}

pub fn configure_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/reward-candidates/review")
            .route(web::get().to(list_platform_reward_candidates)),
    )
    .service(
        web::resource("/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_course_reward_candidate))
            .route(web::get().to(list_course_reward_candidates)),
    )
    .service(
        web::resource("/organizations/{organization_id}/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_organization_reward_candidate)),
    )
    .service(
        web::resource("/courses/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
            .route(web::put().to(decide_reward_candidate_by_teacher)),
    )
    .service(
        web::resource("/reward-candidates/{candidate_id}/amount-decision")
            .route(web::put().to(decide_reward_amount)),
    )
    .service(
        web::resource("/reward-candidates/{candidate_id}/audit")
            .route(web::get().to(list_reward_candidate_audit)),
    );
}

pub fn configure_course_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/reward-candidates")
            .route(web::post().to(submit_course_reward_candidate))
            .route(web::get().to(list_course_reward_candidates)),
    )
    .service(
        web::resource("/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
            .route(web::put().to(decide_reward_candidate_by_teacher)),
    );
}

pub fn configure_organization_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{organization_id}/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_organization_reward_candidate)),
    );
}

pub fn reward_candidate_scope() -> actix_web::Scope {
    web::scope("").configure(configure_reward_candidate_routes)
}
