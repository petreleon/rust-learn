// src/api/mod.rs
pub mod users;
pub mod authentication;
pub mod courses;
pub mod chapters;
pub mod contents;
pub mod organizations;
pub mod roles;
use actix_service::ServiceFactory;
use actix_web::{Scope, dev::ServiceRequest, dev::ServiceResponse, Error};

use crate::middlewares::{conditional_access_middleware::ConditionalAccessMiddleware, jwt_middleware::JwtMiddleware};
use actix_web::web;

pub fn api_scope() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/api")
        .wrap(JwtMiddleware)
        .wrap(ConditionalAccessMiddleware::new(
            |_req: &ServiceRequest| true,
            || actix_web::error::ErrorUnauthorized("Denied by conditional middleware"),
        ))
        .service(users::user_scope())
        .service(authentication::auth_scope())
        .service(courses::course_scope())
        .service(chapters::chapter_scope())
        .service(contents::content_scope())
        .service(organizations::organization_scope())
        .service(roles::roles_scope())
}

