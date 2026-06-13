use actix_web::web;

use crate::http::operations::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health))
        .route("/ready", web::get().to(handlers::readiness));
}

pub fn health_scope() -> actix_web::Scope {
    web::scope("").configure(configure_routes)
}
