use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::role::{CourseRole, OrganizationRole, PlatformRole};
use actix_web::{web, HttpResponse, Responder};

async fn list_platform_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use crate::db::schema::platform_roles::dsl::*;
    use diesel_async::RunQueryDsl;
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let results = platform_roles.load::<PlatformRole>(&mut conn).await;

    match results {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().body("Error loading roles"),
    }
}

async fn list_organization_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use crate::db::schema::organization_roles::dsl::*;
    use diesel_async::RunQueryDsl;
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let results = organization_roles.load::<OrganizationRole>(&mut conn).await;

    match results {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().body("Error loading roles"),
    }
}

async fn list_course_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use crate::db::schema::course_roles::dsl::*;
    use diesel_async::RunQueryDsl;
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let results = course_roles.load::<CourseRole>(&mut conn).await;

    match results {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().body("Error loading roles"),
    }
}

pub fn roles_scope() -> actix_web::Scope {
    web::scope("/roles")
        .service(
            web::resource("").route(web::get().to(list_platform_roles).wrap(
                PlatformPermissionMiddleware::new(Permissions::VIEW_ROLE_ASSIGNMENTS.to_string()),
            )),
        )
        .service(
            web::resource("/organization").route(web::get().to(list_organization_roles).wrap(
                PlatformPermissionMiddleware::new(Permissions::VIEW_ROLE_ASSIGNMENTS.to_string()),
            )),
        )
        .service(
            web::resource("/course").route(web::get().to(list_course_roles).wrap(
                PlatformPermissionMiddleware::new(Permissions::VIEW_ROLE_ASSIGNMENTS.to_string()),
            )),
        )
}
