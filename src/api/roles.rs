use actix_web::{get, web, HttpResponse, Responder};
use crate::models::role::{PlatformRole, OrganizationRole, CourseRole};
use crate::db;


#[get("")]
async fn list_platform_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use diesel_async::RunQueryDsl;
    use crate::db::schema::platform_roles::dsl::*;
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

#[get("/organization")]
async fn list_organization_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use diesel_async::RunQueryDsl;
    use crate::db::schema::organization_roles::dsl::*;
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

#[get("/course")]
async fn list_course_roles(pool: web::Data<db::DbPool>) -> impl Responder {
    use diesel_async::RunQueryDsl;
    use crate::db::schema::course_roles::dsl::*;
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
        .service(list_platform_roles)
        .service(list_organization_roles)
        .service(list_course_roles)
}
