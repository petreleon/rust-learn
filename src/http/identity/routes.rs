use actix_web::web;

use crate::http::identity::{handlers, user_routes};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me").route(web::get().to(handlers::get_current_session)))
        .service(user_routes::user_scope());
}
