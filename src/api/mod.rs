// src/api/mod.rs
pub mod authentication;
pub mod chapters;
pub mod contents;
pub mod courses;
pub mod delegated_permissions;
pub mod health;
pub mod organizations;
pub mod reports;
pub mod reward_candidates;
pub mod reward_fraud_blocks;
pub mod reward_policies;
pub mod roles;
pub mod session;
pub mod teacher_applications;
pub mod users;
pub mod wallets;
use actix_service::ServiceFactory;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, Scope};

use crate::middlewares::{
    conditional_access_middleware::ConditionalAccessMiddleware, jwt_middleware::JwtMiddleware,
};
use actix_web::web;

pub fn api_scope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    web::scope("/api")
        .wrap(JwtMiddleware)
        .wrap(ConditionalAccessMiddleware::new(
            |_req: &ServiceRequest| Box::pin(futures::future::ready(Ok(true))),
            || actix_web::error::ErrorUnauthorized("Denied by conditional middleware"),
        ))
        .service(web::resource("/.well-known/jwks.json").route(web::get().to(authentication::jwks)))
        .service(web::resource("/me").route(web::get().to(session::get_current_session)))
        .service(
            web::resource("/me/preferences")
                .route(web::get().to(session::get_notification_preferences))
                .route(web::put().to(session::save_notification_preferences)),
        )
        .service(web::resource("/me/notifications").route(web::get().to(session::list_notifications)))
        .service(
            web::resource("/me/notifications/{id}/read")
                .route(web::put().to(session::mark_notification_read)),
        )
        .service(users::user_scope())
        .service(authentication::auth_scope())
        .service(courses::course_scope())
        .service(organizations::organization_scope())
        .service(reports::reports_scope())
        .service(delegated_permissions::delegated_permission_scope())
        .configure(reward_candidates::configure_reward_candidate_routes)
        .service(reward_fraud_blocks::reward_fraud_block_scope())
        .service(reward_policies::reward_policy_scope())
        .service(roles::roles_scope())
        .service(teacher_applications::teacher_application_scope())
        .service(wallets::wallet_scope())
}
