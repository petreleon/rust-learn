use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::access_control::list_roles::RoleCatalogUseCase;
use crate::application::access_control::role_catalog::{RoleCatalogEntry, RoleCatalogError};
use crate::http::access_control::dto::RoleResponse;

pub async fn list_platform_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> impl Responder {
    match role_catalog.list_platform_roles().await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(RoleCatalogError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(error) => role_catalog_error_response("platform", &error),
    }
}

pub async fn list_organization_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> impl Responder {
    match role_catalog.list_organization_roles().await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(RoleCatalogError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(error) => role_catalog_error_response("organization", &error),
    }
}

pub async fn list_course_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> impl Responder {
    match role_catalog.list_course_roles().await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(RoleCatalogError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(error) => role_catalog_error_response("course", &error),
    }
}

fn role_responses(roles: Vec<RoleCatalogEntry>) -> Vec<RoleResponse> {
    roles.into_iter().map(RoleResponse::from).collect()
}

fn role_catalog_error_response(scope: &'static str, error: &RoleCatalogError) -> HttpResponse {
    log::error!(
        "event=role_catalog_list_failed scope={} error={}",
        scope,
        role_catalog_error_log(error)
    );
    HttpResponse::InternalServerError().body("Error loading roles")
}

fn role_catalog_error_log(error: &RoleCatalogError) -> String {
    match error {
        RoleCatalogError::Connection(message) | RoleCatalogError::Database(message) => {
            message.clone()
        }
    }
}
