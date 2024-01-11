use actix_web::{Responder, HttpResponse, get, post};
use actix_web::web;

#[get("")]  // This will respond to /user
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Get User")
}

#[post("/add")]  // This will respond to /user/add
async fn add_user() -> impl Responder {
    HttpResponse::Ok().body("Add User")
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(get_user)
        .service(add_user)
        // Add more user services here...
}

// ...more user-related functions...
