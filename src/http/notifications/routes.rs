use actix_web::web;

use crate::http::notifications::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/me/preferences")
            .route(web::get().to(handlers::get_notification_preferences))
            .route(web::put().to(handlers::save_notification_preferences)),
    );
}
