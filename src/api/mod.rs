// src/api/mod.rs
pub mod users;
pub mod authentication;
use actix_service::ServiceFactory;
use actix_web::{Scope, dev::ServiceRequest, dev::ServiceResponse, Error};

use crate::middlewares::{conditional_access_middleware::ConditionalAccessMiddleware, jwt_middleware::JwtMiddleware};
use crate::middlewares::db_connection_middleware::{DB_CONNECTION_MIDDLEWARE_CONDITION, DB_CONNECTION_MIDDLEWARE_ERROR};
use actix_web::web;

pub fn api_scope() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/api")
        .wrap(JwtMiddleware)
        .wrap(ConditionalAccessMiddleware::new(DB_CONNECTION_MIDDLEWARE_CONDITION, DB_CONNECTION_MIDDLEWARE_ERROR))
        .service(users::user_scope())
        .service(authentication::auth_scope())
        // Add more API services here...
}

