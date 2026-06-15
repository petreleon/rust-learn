use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListQuery, OrganizationMemberListUseCase,
};
use crate::http::extractors::auth_user::AuthUser;

use super::dto::OrganizationMemberListParams;
use super::member_dto::OrganizationMemberListResponse;

pub(super) async fn get_organization_members(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationMemberListUseCase>>,
    query: web::Query<OrganizationMemberListParams>,
) -> impl Responder {
    let organization_id = path.into_inner();

    let list_query = OrganizationMemberListQuery::new(
        requester.user_id(),
        organization_id,
        query.search.clone(),
        query.role.clone(),
        query.permission.clone(),
        query.limit,
        query.offset,
    );

    match use_case.list_organization_members(list_query).await {
        Ok(members) => HttpResponse::Ok().json(OrganizationMemberListResponse::from(members)),
        Err(OrganizationMemberListError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization members"),
        Err(OrganizationMemberListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationMemberListError::Connection(error)) => {
            log::error!(
                "event=organization_members_connection_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
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
