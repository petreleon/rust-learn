use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::db::schema::courses;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::course::{Course, UpdateCourse};
use crate::models::param_type::ParamType;
use crate::models::user_jwt::UserJWT;
use crate::repositories::course_repository::assign_role_to_user_in_course;
use crate::services::course_service::{
    create_course_with_invites_for_actor, discover_courses, update_course_for_actor,
    update_course_lifecycle as update_course_lifecycle_status, CourseCreationError,
    CourseDiscoveryQuery, CourseLifecycleError, CourseLifecycleUpdateRequest, CourseUpdateError,
};
use crate::utils::jwt_utils::decode_jwt;
use crate::utils::notifications::NotificationsState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[derive(Deserialize)]
pub struct CourseDiscoveryParams {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn current_user(req: &HttpRequest) -> Result<UserJWT, HttpResponse> {
    req.extensions()
        .get::<UserJWT>()
        .cloned()
        .ok_or_else(|| HttpResponse::Unauthorized().body("Unauthorized access"))
}

fn lifecycle_error_response(error: CourseLifecycleError) -> HttpResponse {
    match error {
        CourseLifecycleError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course status")
        }
        CourseLifecycleError::InvalidStatus(message) => HttpResponse::BadRequest().body(message),
        CourseLifecycleError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseLifecycleError::Database(message) => {
            log::error!("event=course_lifecycle_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course status")
        }
    }
}

fn course_creation_error_response(error: CourseCreationError) -> HttpResponse {
    match error {
        CourseCreationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to create course")
        }
        CourseCreationError::Database(message) => {
            log::error!("event=course_creation_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to create course")
        }
    }
}

fn course_update_error_response(error: CourseUpdateError) -> HttpResponse {
    match error {
        CourseUpdateError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to update course")
        }
        CourseUpdateError::NotFound => HttpResponse::NotFound().body("Course not found"),
        CourseUpdateError::Database(message) => {
            log::error!("event=course_update_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to update course")
        }
    }
}

async fn list_courses(
    pool: web::Data<db::DbPool>,
    query: web::Query<CourseDiscoveryParams>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let discovery = CourseDiscoveryQuery::new(
        query.search.clone(),
        query.organization_id,
        query.limit,
        query.offset,
    );
    let result = discover_courses(&mut conn, discovery).await;

    match result {
        Ok(course_list) => HttpResponse::Ok().json(course_list),
        Err(e) => {
            eprintln!("DB error listing courses: {}", e);
            HttpResponse::InternalServerError().body("Failed to load courses")
        }
    }
}

async fn get_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table
        .find(course_id)
        .first::<Course>(&mut conn)
        .await;

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            eprintln!("DB error fetching course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch course")
        }
    }
}

use crate::db::schema::courses_organizations;
#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub organization_ids: Vec<i32>,
}

async fn create_course(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<CreateCourseRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = create_course_with_invites_for_actor(
        &mut conn,
        requester.user_id,
        body.title.clone(),
        body.organization_ids.clone(),
    )
    .await;

    match result {
        Ok(course) => HttpResponse::Created().json(course),
        Err(error) => course_creation_error_response(error),
    }
}

async fn update_course(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<UpdateCourse>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result =
        update_course_for_actor(&mut conn, requester.user_id, course_id, body.into_inner()).await;

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => course_update_error_response(error),
    }
}

async fn update_course_lifecycle(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseLifecycleUpdateRequest>,
) -> impl Responder {
    let requester = match current_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match update_course_lifecycle_status(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        body.into_inner(),
    )
    .await
    {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => lifecycle_error_response(error),
    }
}

async fn delete_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(courses::table.find(course_id))
        .execute(&mut conn)
        .await;

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

async fn get_course_organizations(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    use crate::models::organization::Organization;

    let result = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .inner_join(crate::db::schema::organizations::table)
        .select(crate::db::schema::organizations::all_columns)
        .load::<Organization>(&mut conn)
        .await;

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

    let mut conn = match pool.get().await {
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
    match assign_role_to_user_in_course(&mut conn, requester_id, target_user_id, course_id, role_name).await {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "course",
                        Some(course_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=course course_id={} target_user_id={} error={:?}",
                        course_id,
                        target_user_id,
                        err
                    );
                }

                if role_name.eq_ignore_ascii_case("STUDENT") {
                    let course_title = courses::table
                        .find(course_id)
                        .select(courses::title)
                        .first::<String>(&mut conn)
                        .await
                        .unwrap_or_else(|_| format!("course #{}", course_id));

                    if let Err(err) = notifications
                        .send_enrollment_notification(target_user_id, course_id, course_title)
                        .await
                    {
                        log::warn!(
                            "event=notification_send_failed kind=enrollment course_id={} target_user_id={} error={:?}",
                            course_id,
                            target_user_id,
                            err
                        );
                    }
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
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
        .configure(crate::api::chapters::config)
        .configure(crate::api::contents::config)
        .service(
            web::resource("")
                .route(
                    web::get()
                        .to(list_courses)
                        .wrap(PlatformPermissionMiddleware::new(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::post().to(create_course)),
        )
        .service(
            web::resource("/{id}/organizations").route(
                web::get()
                    .to(get_course_organizations)
                    .wrap(PlatformPermissionMiddleware::new(
                        Permissions::VIEW_COURSE.to_string(),
                    )),
            ),
        )
        .service(web::resource("/{id}/lifecycle").route(web::put().to(update_course_lifecycle)))
        .service(
            web::resource("/{id}")
                .route(
                    web::get()
                        .to(get_course)
                        .wrap(PlatformPermissionMiddleware::new(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::put().to(update_course))
                .route(
                    web::delete()
                        .to(delete_course)
                        .wrap(CoursePermissionMiddleware::new(
                            Permissions::DELETE_COURSE.to_string(),
                            ParamType::Path,
                            "id".to_string(),
                        )),
                ),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles").route(web::post().to(assign_role).wrap(
                CoursePermissionMiddleware::new(
                    Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            )),
        )
}
