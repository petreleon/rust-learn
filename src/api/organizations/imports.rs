use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::organization::UpdateOrganization;
use crate::http::request_params::ParamType;
use crate::services::course_service::{
    discover_organization_courses, OrganizationCourseListError, OrganizationCourseListQuery,
};
use crate::services::organization_service::{
    self, OrganizationDashboardError, OrganizationMemberListError, OrganizationMemberListQuery,
};
use crate::services::teacher_application_service::{
    OrganizationTeacherApplicationsRequest, TeacherApplicationError,
};
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

#[derive(Deserialize)]
pub struct OrganizationMemberListParams {
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
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
