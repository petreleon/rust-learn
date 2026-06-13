use actix_web::web;

use crate::http::identity::{auth_scope, handlers, jwks, user_routes};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/.well-known/jwks.json").route(web::get().to(jwks)))
        .service(web::resource("/me").route(web::get().to(handlers::get_current_session)))
        .service(auth_scope())
        .service(user_routes::user_scope());
}
