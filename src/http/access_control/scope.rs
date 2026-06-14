use actix_web::web;

use crate::http::access_control::{delegated_permissions, routes};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::roles_scope())
        .service(delegated_permissions::delegated_permission_scope());
}
