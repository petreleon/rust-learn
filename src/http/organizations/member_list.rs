use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::organization_service::{self, OrganizationMemberListError};
use crate::utils::request_auth::authenticated_user;

use super::dto::OrganizationMemberListParams;

pub(super) async fn get_organization_members(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationMemberListParams>,
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

    let list_query = organization_service::OrganizationMemberListQuery::new(
        query.search.clone(),
        query.role.clone(),
        query.permission.clone(),
        query.limit,
        query.offset,
    );

    match organization_service::list_organization_members(
        &mut conn,
        requester.user_id,
        organization_id,
        list_query,
    )
    .await
    {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(OrganizationMemberListError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization members"),
        Err(OrganizationMemberListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationMemberListError::Database(error)) => {
            log::error!(
                "event=organization_members_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization members")
        }
    }
}
