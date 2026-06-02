// src/api/mod.rs
pub mod authentication;
pub mod chapters;
pub mod contents;
pub mod courses;
pub mod health;
pub mod organizations;
pub mod reports;
pub mod roles;
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
        .service(users::user_scope())
        .service(authentication::auth_scope())
        .service(courses::course_scope())
        .service(organizations::organization_scope())
        .service(reports::reports_scope())
        .service(roles::roles_scope())
        .service(wallets::wallet_scope())
}
