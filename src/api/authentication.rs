// src/api/authentication.rs
use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use serde::Deserialize;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::{authentication::Authentication, user_jwt::UserJWT};
use crate::models::user::{User, NewUser};
use crate::db;
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::utils::jwt_utils::create_jwt;
use crate::models::{role::PlatformRole, user_role_platform::UserRolePlatform};

// TODO Add confirmation email on registration

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
}



#[post("/login")]
pub async fn login(
    pool: web::Data<db::DbPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let user_auth_result = User::find_with_password_auth(&req.email, &mut conn);

    match user_auth_result {
        Ok((user, info_auth)) => {
            if let Some(hash) = info_auth {
                if verify(&req.password, &hash).unwrap_or(false) {
                    match create_jwt(user.id()) {
                        Ok(user_jwt) => {
                            HttpResponse::Ok().json(user_jwt) // Return JWT token in response
                        }
                        Err(_) => {
                            HttpResponse::InternalServerError().body("Failed to create JWT")
                        }
                    }
                } else {
                    HttpResponse::Unauthorized().body("Invalid credentials")
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}




#[post("/register")]
pub async fn register(
    pool: web::Data<db::DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    // Create new user
    let new_user_data = NewUser {
        name: req.name.to_string(),
        email: req.email.clone(),
        date_of_birth: req.date_of_birth,
        created_at: chrono::Utc::now().naive_utc(),
        kyc_verified: false,
    };

    let inserted_user = User::create(new_user_data, &mut conn)
        .expect("Error saving new user");

    // Assign default role (STUDENT)
    let role_id = PlatformRole::find_by_name("STUDENT", &mut conn)
        .expect("Error finding STUDENT role");
    
    UserRolePlatform::assign(&mut conn, inserted_user.id(), role_id)
        .expect("Error assigning default role to user");

    // Hash password and create authentication
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    let new_auth = Authentication {
        user_id: inserted_user.id(),
        type_authentication: "password".to_string(),
        info_auth: hashed_password,
    };

    Authentication::create(new_auth, &mut conn)
        .expect("Error saving new authentication");

    HttpResponse::Ok().body("Registration successful")
}

// hello 
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/user_id")]
pub async fn user_id(req: HttpRequest) -> impl Responder {
    // Try to decode the Authorization header to extract the user ID instead of reading request extensions
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str["Bearer ".len()..];
                if let Ok(token_data) = crate::utils::jwt_utils::decode_jwt(token) {
                    let user_jwt = token_data.claims;
                    return HttpResponse::Ok().body(format!("Hello! Your ID is {}", user_jwt.user_id));
                }
            }
        }
    }

    HttpResponse::Ok().body("You didn't provide any ID")
}

// Define the scope for authentication-related routes
pub fn auth_scope() -> actix_web::Scope {
    web::scope("/auth")
    .service(login)
    .service(register)
    .service(hello)
    .service(user_id)
}
