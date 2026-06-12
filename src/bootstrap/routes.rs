use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::api;

#[get("/")]
async fn api_index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "service": "rust-learn-api",
        "health": "/health",
        "readiness": "/ready",
        "api": "/api"
    }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/.well-known/jwks.json",
        web::get().to(api::authentication::jwks),
    )
    .configure(api::health::configure_health_routes)
    .service(api::api_scope())
    .service(api_index);
}
