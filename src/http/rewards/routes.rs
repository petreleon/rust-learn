use actix_web::web;

use crate::http::rewards::handlers::{
    candidate_audit, course_candidates, fraud_block, platform_candidates, reward_history,
    reward_policy, teacher_decision,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reward_policy_scope())
        .service(reward_fraud_block_scope())
        .service(student_reward_history_resource())
        .service(reward_candidate_audit_resource())
        .service(teacher_reward_candidate_decision_resource())
        .service(course_reward_candidates_resource())
        .service(platform_reward_candidates_resource());
}

pub fn reward_policy_scope() -> actix_web::Scope {
    web::scope("/reward-policies").service(
        web::resource("")
            .route(web::post().to(reward_policy::create_reward_policy))
            .route(web::get().to(reward_policy::list_reward_policies)),
    )
}

pub fn reward_fraud_block_scope() -> actix_web::Scope {
    web::scope("/reward-fraud-blocks")
        .service(
            web::resource("")
                .route(web::post().to(fraud_block::create_reward_fraud_block))
                .route(web::get().to(fraud_block::list_reward_fraud_blocks)),
        )
        .service(
            web::resource("/{id}/revoke")
                .route(web::put().to(fraud_block::revoke_reward_fraud_block)),
        )
        .service(
            web::resource("/{id}/audit")
                .route(web::get().to(fraud_block::reward_fraud_block_audit_history)),
        )
}

pub fn student_reward_history_resource() -> actix_web::Resource {
    web::resource("/reward-candidates/me/history")
        .route(web::get().to(reward_history::list_my_reward_history))
}

pub fn reward_candidate_audit_resource() -> actix_web::Resource {
    web::resource("/reward-candidates/{candidate_id}/audit")
        .route(web::get().to(candidate_audit::list_reward_candidate_audit))
}

pub fn course_reward_candidates_resource() -> actix_web::Resource {
    web::resource("/courses/{course_id}/reward-candidates")
        .route(web::get().to(course_candidates::list_course_reward_candidates))
}

pub fn platform_reward_candidates_resource() -> actix_web::Resource {
    web::resource("/reward-candidates/review")
        .route(web::get().to(platform_candidates::list_platform_reward_candidates))
}

pub fn teacher_reward_candidate_decision_resource() -> actix_web::Resource {
    web::resource("/courses/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
        .route(web::put().to(teacher_decision::decide_reward_candidate_by_teacher))
}

pub fn course_scope_teacher_reward_candidate_decision_resource() -> actix_web::Resource {
    web::resource("/{course_id}/reward-candidates/{candidate_id}/teacher-decision")
        .route(web::put().to(teacher_decision::decide_reward_candidate_by_teacher))
}
