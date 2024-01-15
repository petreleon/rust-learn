pub mod users;
pub mod authentication;
use actix_web::web;

pub fn api_scope() -> actix_web::Scope {
    web::scope("/api")
        .service(users::user_scope())
        .service(authentication::auth_scope())
        // Add more API services here...
}

