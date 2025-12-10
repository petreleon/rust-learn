use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use serde::Deserialize;
use crate::db;
use crate::models::course::{Course, NewCourse, UpdateCourse};
use crate::db::schema::courses;
use crate::utils::jwt_utils::decode_jwt;
use crate::utils::db_utils::course::assign_role_to_user_in_course;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::config::constants::permissions::Permissions;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[get("")]
async fn list_courses(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table.load::<Course>(&mut conn);

    match result {
        Ok(course_list) => HttpResponse::Ok().json(course_list),
        Err(e) => {
            eprintln!("DB error listing courses: {}", e);
            HttpResponse::InternalServerError().body("Failed to load courses")
        }
    }
}

#[get("/{id}")]
async fn get_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table.find(course_id).first::<Course>(&mut conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            eprintln!("DB error fetching course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch course")
        }
    }
}

use crate::models::courses_organizations::NewCourseOrganization;
use crate::db::schema::courses_organizations;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub organization_ids: Vec<i32>,
}

#[post("")]
async fn create_course(pool: web::Data<db::DbPool>, req: web::Json<CreateCourseRequest>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = crate::utils::course_utils::create_course_with_invites(
        &mut conn,
        req.title.clone(),
        req.organization_ids.clone(),
    );

    match result {
        Ok(course) => HttpResponse::Created().json(course),
        Err(e) => {
            eprintln!("DB error creating course: {}", e);
            HttpResponse::InternalServerError().body("Failed to create course")
        }
    }
}

async fn update_course(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    req: web::Json<UpdateCourse>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(courses::table.find(course_id))
        .set(&*req)
        .get_result::<Course>(&mut conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            eprintln!("DB error updating course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to update course")
        }
    }
}

async fn delete_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(courses::table.find(course_id))
        .execute(&mut conn);

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Course deleted")
            } else {
                HttpResponse::NotFound().body("Course not found")
            }
        }
        Err(e) => {
            eprintln!("DB error deleting course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to delete course")
        }
    }
}

#[get("/{id}/organizations")]
async fn get_course_organizations(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    use crate::models::organization::Organization;
    
    let result = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .inner_join(crate::db::schema::organizations::table)
        .select(crate::db::schema::organizations::all_columns)
        .load::<Organization>(&mut conn);

    match result {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(e) => {
            eprintln!("DB error fetching course organizations: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch course organizations")
        }
    }
}

async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (course_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

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

    // Permission Check: Handled by Middleware
    // Middleware "MANAGE_COURSE_ENROLLMENTS" required.

    // Perform Assignment with Hierarchy Check
    match assign_role_to_user_in_course(&mut conn, requester_id, target_user_id, course_id, role_name) {
        Ok(_) => HttpResponse::Ok().body("Role assigned successfully"),
        Err(diesel::result::Error::RollbackTransaction) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(diesel::result::Error::NotFound) => HttpResponse::BadRequest().body("Role or User not found"),
        Err(e) => {
            eprintln!("Error assigning role: {}", e);
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}

pub fn course_scope() -> actix_web::Scope {
    web::scope("/courses")
        .service(list_courses)
        .service(get_course)
        .service(create_course)
        .service(get_course_organizations)
        .service(
             web::resource("/{id}")
                .route(web::put().to(update_course).wrap(CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
                .route(web::delete().to(delete_course).wrap(CoursePermissionMiddleware::new(
                    Permissions::DELETE_COURSE.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles")
                .route(web::post().to(assign_role).wrap(CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                )))
        )
}
