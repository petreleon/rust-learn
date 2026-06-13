use actix_web::web;

use crate::http::rewards::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reward_policy_scope());
}

pub fn reward_policy_scope() -> actix_web::Scope {
    web::scope("/reward-policies").service(
        web::resource("")
            .route(web::post().to(handlers::create_reward_policy))
            .route(web::get().to(handlers::list_reward_policies)),
    )
}
