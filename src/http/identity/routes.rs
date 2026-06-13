use actix_web::web;

use crate::http::identity::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me").route(web::get().to(handlers::get_current_session)));
}
