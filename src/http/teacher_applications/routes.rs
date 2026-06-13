use actix_web::web;

use crate::http::teacher_applications::{audit, handlers, platform_review};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(teacher_application_scope());
}

fn teacher_application_scope() -> actix_web::Scope {
    web::scope("/teacher-applications")
        .service(
            web::resource("")
                .route(web::post().to(handlers::submit_application))
                .route(web::get().to(handlers::list_applications)),
        )
        .service(
            web::resource("/review")
                .route(web::get().to(platform_review::list_platform_review_applications)),
        )
        .service(web::resource("/me").route(web::get().to(handlers::get_my_application)))
        .service(web::resource("/{id}/decision").route(web::put().to(handlers::decide_application)))
        .service(web::resource("/{id}/audit").route(web::get().to(audit::list_audit_events)))
}
