use actix_web::web;

use crate::http::wallet::handlers::{audit, read};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me").route(web::get().to(read::get_my_wallet)))
        .service(web::resource("/me/audit").route(web::get().to(audit::get_my_wallet_audit)))
        .service(web::resource("/users/{id}").route(web::get().to(read::get_user_wallet)))
        .service(
            web::resource("/users/{id}/audit").route(web::get().to(audit::get_user_wallet_audit)),
        )
        .service(
            web::resource("/organizations/{id}")
                .route(web::get().to(read::get_organization_wallet)),
        )
        .service(
            web::resource("/organizations/{id}/audit")
                .route(web::get().to(audit::get_organization_wallet_audit)),
        );
}
