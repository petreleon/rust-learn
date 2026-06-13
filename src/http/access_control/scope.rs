use actix_web::web;

use crate::http::access_control::routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::roles_scope());
}
