use actix_web::{Responder, HttpResponse, get, post};
use actix_web::{web, HttpRequest};
use serde_json::json;
use diesel::prelude::*;
use crate::db;
use crate::models::user::User;
use serde::Deserialize;
use crate::models::user_role_platform::UserRolePlatform;
use crate::models::role::PlatformRole;
use crate::models::role_platform_hierarchy::RolePlatformHierarchy;
use crate::utils::jwt_utils::decode_jwt;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

// GET /user -> list users (placeholder implementation)
#[get("")]
async fn list_users(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = User::find_all(&mut conn);

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

    let result = User::find_by_id(user_id, &mut conn);

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

// POST /user/{id}/role -> assign role to user
#[post("/{id}/role")]
async fn assign_role(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let target_user_id = path.into_inner();
    let role_name = &body.role_name;

    // 1. Get DB connection
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    // 2. Identify Requester from JWT
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().body("Missing Authorization header"),
    };
    
    let token = if auth_header.starts_with("Bearer ") {
        &auth_header["Bearer ".len()..]
    } else {
        return HttpResponse::Unauthorized().body("Invalid Authorization header format");
    };

    let requester_id = match decode_jwt(token) {
        Ok(data) => data.claims.user_id,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    // 3. Permission Check: ASSIGN_ROLES_TO_USER
    match UserRolePlatform::has_permission(&mut conn, requester_id, "ASSIGN_ROLES_TO_USER") {
        Ok(true) => (), // Allowed
        Ok(false) => return HttpResponse::Forbidden().body("Missing permission: ASSIGN_ROLES_TO_USER"),
        Err(_) => return HttpResponse::InternalServerError().body("Error checking permissions"),
    }

    // 4. Hierarchy Checks
    // 4a. Get Requester Rank (lower is better, 0 is best)
    let requester_level = match RolePlatformHierarchy::get_min_level(&mut conn, requester_id) {
        Ok(Some(lvl)) => lvl,
        Ok(None) => return HttpResponse::Forbidden().body("Requester has no hierarchical rank"),
        Err(_) => return HttpResponse::InternalServerError().body("Error fetching requester rank"),
    };

    // 4b. Get Target User Rank (if any)
    let target_level = match RolePlatformHierarchy::get_min_level(&mut conn, target_user_id) {
        Ok(Some(lvl)) => lvl,
        // If target has no roles, they are level "infinity" (e.g. max i32) effectively, 
        // so they are definitely lower rank than requester. We permit modification.
        Ok(None) => i32::MAX, 
        Err(_) => return HttpResponse::InternalServerError().body("Error fetching target rank"),
    };

    // Rule 1: Cannot modify someone ranked higher or equal to you
    // Note: If target_level == requester_level, strictly preventing modification avoids infighting.
    if target_level <= requester_level {
        return HttpResponse::Forbidden().body("Cannot modify a user with equal or higher rank");
    }

    // 4c. Get New Role Rank
    // Find role ID first
    let role_id = match PlatformRole::find_by_name(role_name, &mut conn) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body(format!("Role '{}' not found", role_name)),
    };

    // Get rank of this new specific role
    // Using RolePlatformHierarchy table directly for this role
    let new_role_level = {
        use crate::db::schema::role_platform_hierarchy::dsl::*;
        role_platform_hierarchy
            .filter(platform_role_id.eq(Some(role_id)))
            .select(hierarchy_level)
            .first::<i32>(&mut conn)
            .unwrap_or(i32::MAX) // If role has no hierarchy entry, assume lowest rank? Or forbidden?
                                 // Let's assume strict: if not in hierarchy, maybe safe, but safer to treat as high or error?
                                 // Given migration 2025-08-12, all roles have hierarchy. If missing, something is wrong.
    };

    // Rule 2: Cannot assign a role ranked higher or equal to yourself
    if new_role_level <= requester_level {
        return HttpResponse::Forbidden().body("Cannot assign a role with equal or higher rank than yourself");
    }

    // 5. Perform Assignment
    match UserRolePlatform::assign(&mut conn, target_user_id, role_id) {
        Ok(_) => HttpResponse::Ok().body("Role assigned successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to assign role"),
    }
}

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(list_users)
        .service(get_user)
        .service(assign_role)
}
