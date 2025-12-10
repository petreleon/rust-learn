use actix_web::{get, web, HttpResponse, Responder};
use crate::models::role::PlatformRole;
use crate::db;
use diesel::prelude::*;
use crate::db::schema::platform_roles::dsl::*;

#[get("")]
async fn list_platform_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let results = platform_roles.load::<PlatformRole>(&mut conn);

    match results {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().body("Error loading roles"),
    }
}

pub fn roles_scope() -> actix_web::Scope {
    web::scope("/roles")
        .service(list_platform_roles)
}
