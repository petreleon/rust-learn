use crate::application::access_control::list_roles;
use crate::application::access_control::role_catalog::RoleCatalogError;
use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::http::access_control::dto::RoleResponse;
use crate::infra::postgres::access_control::role_catalog_store::PostgresRoleCatalogStore;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use actix_web::{web, HttpResponse, Responder};

async fn list_platform_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresRoleCatalogStore::new(&mut conn);

    match list_roles::list_platform_roles(&mut store).await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(error) => {
            log::error!(
                "event=role_catalog_list_failed scope=platform error={}",
                role_catalog_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Error loading roles")
        }
    }
}

async fn list_organization_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresRoleCatalogStore::new(&mut conn);

    match list_roles::list_organization_roles(&mut store).await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(error) => {
            log::error!(
                "event=role_catalog_list_failed scope=organization error={}",
                role_catalog_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Error loading roles")
        }
    }
}

async fn list_course_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresRoleCatalogStore::new(&mut conn);

    match list_roles::list_course_roles(&mut store).await {
        Ok(roles) => HttpResponse::Ok().json(role_responses(roles)),
        Err(error) => {
            log::error!(
                "event=role_catalog_list_failed scope=course error={}",
                role_catalog_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Error loading roles")
        }
    }
}

fn role_responses(
    roles: Vec<crate::application::access_control::role_catalog::RoleCatalogEntry>,
) -> Vec<RoleResponse> {
    roles.into_iter().map(RoleResponse::from).collect()
}

fn role_catalog_error_log(error: &RoleCatalogError) -> String {
    match error {
        RoleCatalogError::Database(message) => message.clone(),
    }
}

pub fn roles_scope() -> actix_web::Scope {
    web::scope("/roles")
        .service(
            web::resource("").route(web::get().to(list_platform_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            )),
        )
        .service(
            web::resource("/organization").route(web::get().to(list_organization_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            )),
        )
        .service(
            web::resource("/course").route(web::get().to(list_course_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            )),
        )
}
