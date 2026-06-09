use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::organization::UpdateOrganization;
use crate::models::param_type::ParamType;
use crate::services::course_service::{
    discover_organization_courses, OrganizationCourseListError, OrganizationCourseListQuery,
};
use crate::services::organization_service;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::{authenticated_user, authenticated_user_id};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[derive(Deserialize)]
pub struct OrganizationCourseListParams {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

async fn list_organizations(pool: web::Data<db::DbPool>) -> impl Responder {
    match organization_service::list_organizations(&pool).await {
        Ok(org_list) => HttpResponse::Ok().json(org_list),
        Err(e) => {
            log::error!("event=organization_list_failed error={}", e);
            HttpResponse::InternalServerError().body("Failed to load organizations")
        }
    }
}

async fn get_organization(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    match organization_service::get_organization(&pool, org_id).await {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(e) => {
            log::error!(
                "event=organization_fetch_failed organization_id={} error={}",
                org_id,
                e
            );
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

async fn create_organization(
    pool: web::Data<db::DbPool>,
    req: web::Json<CreateOrganizationRequest>,
) -> impl Responder {
    let dto = organization_service::CreateOrganizationDto {
        name: req.name.clone(),
        website_link: req.website_link.clone(),
        profile_url: req.profile_url.clone(),
        course_ids: req.course_ids.clone(),
    };

    match organization_service::create_organization(&pool, dto).await {
        Ok(org) => HttpResponse::Created().json(org),
        Err(e) => {
            log::error!("event=organization_create_failed error={}", e);
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
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(e) => {
            log::error!(
                "event=organization_update_failed organization_id={} error={}",
                org_id,
                e
            );
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
            log::error!(
                "event=organization_delete_failed organization_id={} error={}",
                org_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to delete organization")
        }
    }
}

async fn get_organization_courses(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationCourseListParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let list_query = OrganizationCourseListQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_organization_courses(&mut conn, requester.user_id, organization_id, list_query)
        .await
    {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(OrganizationCourseListError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization courses"),
        Err(OrganizationCourseListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationCourseListError::Database(error)) => {
            log::error!(
                "event=organization_courses_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
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
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match organization_service::assign_role(&pool, requester_id, target_user_id, org_id, role_name)
        .await
    {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "organization",
                        Some(org_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=organization organization_id={} target_user_id={} error={:?}",
                        org_id,
                        target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(msg) => {
            if msg.contains("Hierarchy check failed") {
                HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.")
            } else if msg.contains("Role or User not found") {
                HttpResponse::BadRequest().body(msg)
            } else {
                log::error!(
                    "event=organization_role_assign_failed organization_id={} requester_user_id={} target_user_id={} role={} error={}",
                    org_id,
                    requester_id,
                    target_user_id,
                    role_name,
                    msg
                );
                HttpResponse::InternalServerError().body("Failed to assign role")
            }
        }
    }
}

pub fn organization_scope() -> actix_web::Scope {
    web::scope("/organizations")
        .configure(crate::api::reward_candidates::configure_organization_reward_candidate_routes)
        .service(
            web::resource("")
                .route(web::get().to(list_organizations).wrap(
                    PlatformPermissionMiddleware::require(
                        Permissions::VIEW_ORGANIZATION.to_string(),
                    ),
                ))
                .route(web::post().to(create_organization).wrap(
                    PlatformPermissionMiddleware::require(
                        Permissions::CREATE_ORGANIZATION.to_string(),
                    ),
                )),
        )
        .service(web::resource("/{id}/courses").route(web::get().to(get_organization_courses)))
        .service(
            web::resource("/{id}")
                .route(
                    web::get()
                        .to(get_organization)
                        .wrap(PlatformPermissionMiddleware::require(
                            Permissions::VIEW_ORGANIZATION.to_string(),
                        )),
                )
                .route(web::put().to(update_organization).wrap(
                    OrganizationPermissionMiddleware::require(
                        Permissions::MANAGE_ORG_SETTINGS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                ))
                .route(web::delete().to(delete_organization).wrap(
                    OrganizationPermissionMiddleware::require(
                        Permissions::MANAGE_ORG_SETTINGS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                )),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles").route(web::post().to(assign_role).wrap(
                OrganizationPermissionMiddleware::require(
                    Permissions::ASSIGN_ROLES_TO_ORG_USERS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            )),
        )
        .service(
            web::resource("/{id}/teacher-applications").route(
                web::post()
                    .to(crate::api::teacher_applications::nominate_application)
                    .wrap(OrganizationPermissionMiddleware::require(
                        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    )),
            ),
        )
}
