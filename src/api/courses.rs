use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::db::schema::{course_join_requests, courses};
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::course::{Course, UpdateCourse};
use crate::models::course_join_request::COURSE_JOIN_STATUS_APPROVED;
use crate::models::param_type::ParamType;
use crate::repositories::course_repository::assign_role_to_user_in_course;
use crate::services::course_enrollment_service::{
    decide_course_join_request as decide_course_join_request_for_actor,
    remove_course_enrollment as remove_course_enrollment_for_actor,
    request_course_join as request_course_join_for_actor, CourseEnrollmentError,
    CourseJoinDecisionRequest,
};
use crate::services::course_service::{
    create_course_with_invites_for_actor, discover_courses, discover_learner_course_catalog,
    discover_teacher_course_dashboard, get_learner_course_detail, get_learner_course_learning,
    update_course_for_actor, update_course_lifecycle as update_course_lifecycle_status,
    CourseCreationError, CourseDiscoveryQuery, CourseLifecycleError, CourseLifecycleUpdateRequest,
    CourseUpdateError, LearnerCourseCatalogError, LearnerCourseCatalogQuery,
    TeacherCourseDashboardError, TeacherCourseDashboardQuery,
};
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::{authenticated_user, authenticated_user_id};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
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

#[derive(Deserialize)]
pub struct LearnerCourseCatalogParams {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct TeacherCourseDashboardParams {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
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

fn course_enrollment_error_response(error: CourseEnrollmentError) -> HttpResponse {
    match error {
        CourseEnrollmentError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to manage enrollment")
        }
        CourseEnrollmentError::InvalidStatus(message) => HttpResponse::BadRequest().body(message),
        CourseEnrollmentError::NotFound => {
            HttpResponse::NotFound().body("Course enrollment not found")
        }
        CourseEnrollmentError::Database(message) => {
            log::error!("event=course_enrollment_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to manage course enrollment")
        }
    }
}

fn learner_course_catalog_error_response(error: LearnerCourseCatalogError) -> HttpResponse {
    match error {
        LearnerCourseCatalogError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have permission to view course content")
        }
        LearnerCourseCatalogError::NotFound => HttpResponse::NotFound().body("Course not found"),
        LearnerCourseCatalogError::Database(message) => {
            log::error!("event=learner_course_catalog_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load course catalog")
        }
    }
}

fn teacher_course_dashboard_error_response(error: TeacherCourseDashboardError) -> HttpResponse {
    match error {
        TeacherCourseDashboardError::Database(message) => {
            log::error!("event=teacher_course_dashboard_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to load teaching courses")
        }
    }
}

async fn send_enrollment_notification_for_course(
    conn: &mut AsyncPgConnection,
    notifications: &NotificationsState,
    target_user_id: i32,
    course_id: i32,
) {
    let course_title = courses::table
        .find(course_id)
        .select(courses::title)
        .first::<String>(conn)
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

async fn list_learner_course_catalog(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<LearnerCourseCatalogParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let catalog_query = LearnerCourseCatalogQuery::new(
        query.search.clone(),
        query.organization_id,
        query.lifecycle_status.clone(),
        query.enrollment_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_learner_course_catalog(&mut conn, requester.user_id, catalog_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

async fn list_teacher_course_dashboard(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<TeacherCourseDashboardParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let dashboard_query = TeacherCourseDashboardQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.limit,
        query.offset,
    );

    match discover_teacher_course_dashboard(&mut conn, requester.user_id, dashboard_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

async fn get_learner_course_learning_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_course_learning(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(learning) => HttpResponse::Ok().json(learning),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

async fn get_learner_course_catalog_detail(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_course_detail(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(error) => learner_course_catalog_error_response(error),
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
            log::error!(
                "event=course_list_failed search={:?} organization_id={:?} limit={:?} offset={:?} error={}",
                query.search,
                query.organization_id,
                query.limit,
                query.offset,
                e
            );
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
            log::error!(
                "event=course_fetch_failed course_id={} error={}",
                course_id,
                e
            );
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
    let requester = match authenticated_user(&req) {
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
    let requester = match authenticated_user(&req) {
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
    let requester = match authenticated_user(&req) {
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

async fn request_course_join(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match request_course_join_for_actor(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(join_request) => HttpResponse::Created().json(join_request),
        Err(error) => course_enrollment_error_response(error),
    }
}

async fn decide_course_join_request(
    req: HttpRequest,
    path: web::Path<(i32, i64)>,
    pool: web::Data<db::DbPool>,
    body: web::Json<CourseJoinDecisionRequest>,
) -> impl Responder {
    let reviewer = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, request_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let status_before_decision = course_join_requests::table
        .find(request_id)
        .select(course_join_requests::status)
        .first::<String>(&mut conn)
        .await
        .optional()
        .ok()
        .flatten();

    match decide_course_join_request_for_actor(
        &mut conn,
        reviewer.user_id,
        course_id,
        request_id,
        body.into_inner(),
    )
    .await
    {
        Ok(join_request) => {
            if join_request.status == COURSE_JOIN_STATUS_APPROVED
                && status_before_decision.as_deref() != Some(COURSE_JOIN_STATUS_APPROVED)
            {
                if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                    send_enrollment_notification_for_course(
                        &mut conn,
                        notifications,
                        join_request.requester_user_id,
                        join_request.course_id,
                    )
                    .await;
                }
            }

            HttpResponse::Ok().json(join_request)
        }
        Err(error) => course_enrollment_error_response(error),
    }
}

async fn remove_course_enrollment(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let actor = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (course_id, target_user_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match remove_course_enrollment_for_actor(&mut conn, actor.user_id, course_id, target_user_id)
        .await
    {
        Ok(removal) => HttpResponse::Ok().json(removal),
        Err(error) => course_enrollment_error_response(error),
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
            log::error!(
                "event=course_delete_failed course_id={} error={}",
                course_id,
                e
            );
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
            log::error!(
                "event=course_organizations_fetch_failed course_id={} error={}",
                course_id,
                e
            );
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
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
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

            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(diesel::result::Error::RollbackTransaction) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(diesel::result::Error::NotFound) => HttpResponse::BadRequest().body("Role or User not found"),
        Err(e) => {
            log::error!(
                "event=course_role_assign_failed requester_user_id={} target_user_id={} course_id={} role={} error={}",
                requester_id,
                target_user_id,
                course_id,
                role_name,
                e
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}

pub fn course_scope() -> actix_web::Scope {
    web::scope("/courses")
        .configure(crate::api::chapters::config)
        .configure(crate::api::contents::config)
        .configure(crate::api::reward_candidates::configure_course_reward_candidate_routes)
        .service(web::resource("/catalog").route(web::get().to(list_learner_course_catalog)))
        .service(web::resource("/teaching").route(web::get().to(list_teacher_course_dashboard)))
        .service(
            web::resource("/catalog/{id}/learn")
                .route(web::get().to(get_learner_course_learning_route)),
        )
        .service(
            web::resource("/catalog/{id}").route(web::get().to(get_learner_course_catalog_detail)),
        )
        .service(
            web::resource("")
                .route(
                    web::get()
                        .to(list_courses)
                        .wrap(PlatformPermissionMiddleware::require(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::post().to(create_course)),
        )
        .service(
            web::resource("/{id}/organizations").route(
                web::get().to(get_course_organizations).wrap(
                    PlatformPermissionMiddleware::require(Permissions::VIEW_COURSE.to_string()),
                ),
            ),
        )
        .service(web::resource("/{id}/lifecycle").route(web::put().to(update_course_lifecycle)))
        .service(web::resource("/{id}/join-requests").route(web::post().to(request_course_join)))
        .service(
            web::resource("/{id}/join-requests/{request_id}/decision")
                .route(web::put().to(decide_course_join_request)),
        )
        .service(
            web::resource("/{id}/enrollments/{user_id}")
                .route(web::delete().to(remove_course_enrollment)),
        )
        .service(
            web::resource("/{id}")
                .route(
                    web::get()
                        .to(get_course)
                        .wrap(PlatformPermissionMiddleware::require(
                            Permissions::VIEW_COURSE.to_string(),
                        )),
                )
                .route(web::put().to(update_course))
                .route(
                    web::delete()
                        .to(delete_course)
                        .wrap(CoursePermissionMiddleware::require(
                            Permissions::DELETE_COURSE.to_string(),
                            ParamType::Path,
                            "id".to_string(),
                        )),
                ),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles").route(web::post().to(assign_role).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            )),
        )
}
