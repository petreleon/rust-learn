pub fn configure_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_course_reward_candidate)),
    )
    .service(
        web::resource("/organizations/{organization_id}/courses/{course_id}/reward-candidates")
            .route(web::post().to(submit_organization_reward_candidate)),
    )
    .service(
        web::resource("/reward-candidates/{candidate_id}/amount-decision")
            .route(web::put().to(decide_reward_amount)),
    );
}

pub fn configure_course_reward_candidate_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/reward-candidates")
            .route(web::post().to(submit_course_reward_candidate)),
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
