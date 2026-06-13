use actix_web::{web, HttpResponse, Responder};

use crate::db;
use crate::models::organization::UpdateOrganization;
use crate::services::organization_service;

use super::dto::CreateOrganizationRequest;

pub(super) async fn list_organizations(pool: web::Data<db::DbPool>) -> impl Responder {
    match organization_service::list_organizations(&pool).await {
        Ok(org_list) => HttpResponse::Ok().json(org_list),
        Err(e) => {
            log::error!("event=organization_list_failed error={}", e);
            HttpResponse::InternalServerError().body("Failed to load organizations")
        }
    }
}

pub(super) async fn get_organization(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
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

pub(super) async fn create_organization(
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

pub(super) async fn update_organization(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    req: web::Json<UpdateOrganization>,
) -> impl Responder {
    let org_id = path.into_inner();
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

pub(super) async fn delete_organization(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
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
