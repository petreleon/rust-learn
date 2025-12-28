use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use serde::Deserialize;
use crate::db;
use crate::models::organization::UpdateOrganization;
use crate::utils::jwt_utils::decode_jwt;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::config::constants::permissions::Permissions;
use crate::services::organization_service;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[get("")]
async fn list_organizations(pool: web::Data<db::DbPool>) -> impl Responder {
    match organization_service::list_organizations(&pool).await {
        Ok(org_list) => HttpResponse::Ok().json(org_list),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Failed to load organizations")
        }
    }
}

#[get("/{id}")]
async fn get_organization(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    match organization_service::get_organization(&pool, org_id).await {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => {
            eprintln!("DB error fetching organization {}: {}", org_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch organization")
        }
    }
}

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}

#[post("")]
async fn create_organization(pool: web::Data<db::DbPool>, req: web::Json<CreateOrganizationRequest>) -> impl Responder {
    let dto = organization_service::CreateOrganizationDto {
        name: req.name.clone(),
        website_link: req.website_link.clone(),
        profile_url: req.profile_url.clone(),
        course_ids: req.course_ids.clone(),
    };

    match organization_service::create_organization(&pool, dto).await {
        Ok(org) => HttpResponse::Created().json(org),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Failed to create organization")
        }
    }
}

async fn update_organization(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    req: web::Json<UpdateOrganization>,
) -> impl Responder {
    let org_id = path.into_inner();
    // Using into_inner() on Json wrapper to get the inner struct
    let update_data = req.into_inner(); 
    
    match organization_service::update_organization(&pool, org_id, update_data).await {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => {
            eprintln!("DB error updating organization {}: {}", org_id, e);
            HttpResponse::InternalServerError().body("Failed to update organization")
        }
    }
}

async fn delete_organization(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    match organization_service::delete_organization(&pool, org_id).await {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Organization deleted")
            } else {
                HttpResponse::NotFound().body("Organization not found")
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Failed to delete organization")
        }
    }
}

#[get("/{id}/courses")]
async fn get_organization_courses(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    match organization_service::get_organization_courses(&pool, org_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("Failed to fetch organization courses")
        }
    }
}

async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (org_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    // Identify Requester from JWT
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

    match organization_service::assign_role(&pool, requester_id, target_user_id, org_id, role_name).await {
        Ok(_) => HttpResponse::Ok().body("Role assigned successfully"),
        Err(msg) => {
             if msg.contains("Hierarchy check failed") {
                 HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.")
             } else if msg.contains("Role or User not found") {
                 HttpResponse::BadRequest().body(msg)
             } else {
                 eprintln!("{}", msg);
                 HttpResponse::InternalServerError().body("Failed to assign role")
             }
        }
    }
}

pub fn organization_scope() -> actix_web::Scope {
    web::scope("/organizations")
        .service(list_organizations)
        .service(get_organization)
        .service(create_organization)
        .service(get_organization_courses)
        .service(
            web::resource("/{id}")
                .route(web::put().to(update_organization).wrap(OrganizationPermissionMiddleware::new(
                    Permissions::MANAGE_ORG_SETTINGS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
                .route(web::delete().to(delete_organization).wrap(OrganizationPermissionMiddleware::new(
                    Permissions::MANAGE_ORG_SETTINGS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles")
                .route(web::post().to(assign_role).wrap(OrganizationPermissionMiddleware::new(
                    Permissions::ASSIGN_ROLES_TO_ORG_USERS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
        )
}
