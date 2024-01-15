use actix_web::{web, HttpResponse, Responder, post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    // Define the fields for login request
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    // Define the fields for registration request
}

// Define your authentication-related routes here
#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    // Implement your login logic here
    HttpResponse::Ok().finish()
}

#[post("/register")]
pub async fn register(req: web::Json<RegisterRequest>) -> impl Responder {
    // Implement your registration logic here
    HttpResponse::Ok().finish()
}

// Define the scope for authentication-related routes
pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
        // The attribute macros on the functions automatically define the routes
        .service(login)
        .service(register)
}
