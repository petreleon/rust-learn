use actix_web::web;

use crate::http::notifications::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/me/preferences")
            .route(web::get().to(handlers::get_notification_preferences))
            .route(web::put().to(handlers::save_notification_preferences)),
    )
    .service(
        web::resource("/me/notifications")
            .route(web::get().to(handlers::list_notifications))
            .route(web::delete().to(handlers::clear_notifications)),
    )
    .service(
        web::resource("/me/notifications/{id}/read")
            .route(web::put().to(handlers::mark_notification_read)),
    );
}
