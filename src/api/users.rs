use actix_web::{Responder, HttpResponse, get};
use actix_web::web;
use serde_json::json;
use diesel::prelude::*;
use crate::db;
use crate::db::schema::users::dsl as users_dsl;
use crate::models::user::User;

// GET /user -> list users (placeholder implementation)
#[get("")]
async fn list_users(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = users_dsl::users.load::<User>(&mut conn);

    match result {
        Ok(user_list) => {
            let users_json: Vec<_> = user_list.iter().map(|u| {
                json!({
                    "id": u.id,
                    "name": u.name,
                    "email": u.email,
                    "date_of_birth": u.date_of_birth.map(|d| d.to_string()),
                    "created_at": u.created_at.to_string(),
                    "kyc_verified": u.kyc_verified,
                })
            }).collect();
            HttpResponse::Ok().json(json!({ "users": users_json }))
        }
        Err(e) => {
            eprintln!("DB error listing users: {}", e);
            HttpResponse::InternalServerError().body("Failed to load users")
        }
    }
}

// GET /user/{id} -> get a single user by id (placeholder)
#[get("/{id}")]
async fn get_user(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = users_dsl::users.filter(users_dsl::id.eq(user_id)).first::<User>(&mut conn);

    match result {
        Ok(u) => HttpResponse::Ok().json(json!({
            "id": u.id,
            "name": u.name,
            "email": u.email,
            "date_of_birth": u.date_of_birth.map(|d| d.to_string()),
            "created_at": u.created_at.to_string(),
            "kyc_verified": u.kyc_verified,
        })),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("DB error fetching user {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(list_users)
        .service(get_user)
        
        // Note: no public POST /add route here — user creation must be consensual.
}

// ...more user-related functions...
