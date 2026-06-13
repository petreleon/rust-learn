use actix_web::web;

use crate::http::rewards::handlers::{fraud_block, reward_history, reward_policy};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reward_policy_scope())
        .service(reward_fraud_block_scope())
        .service(student_reward_history_resource());
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
