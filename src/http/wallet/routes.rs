use actix_web::web;

use crate::http::wallet::handlers;

pub fn configure_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me/audit").route(web::get().to(handlers::get_my_wallet_audit)))
        .service(
            web::resource("/users/{id}/audit")
                .route(web::get().to(handlers::get_user_wallet_audit)),
        )
        .service(
            web::resource("/organizations/{id}/audit")
                .route(web::get().to(handlers::get_organization_wallet_audit)),
        );
}
