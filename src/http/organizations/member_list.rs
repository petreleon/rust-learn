use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListQuery, OrganizationMemberListUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::dto::OrganizationMemberListParams;
use super::errors::organization_member_list_error;
use super::member_dto::OrganizationMemberListResponse;

pub(super) async fn get_organization_members(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationMemberListUseCase>>,
    query: web::Query<OrganizationMemberListParams>,
) -> Result<web::Json<OrganizationMemberListResponse>, ApiError> {
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

    use_case
        .list_organization_members(list_query)
        .await
        .map(OrganizationMemberListResponse::from)
        .map(web::Json)
        .map_err(|error| organization_member_list_error(organization_id, error))
}
