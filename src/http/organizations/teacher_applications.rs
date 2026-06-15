use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationListQuery, OrganizationTeacherApplicationListUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::dto::OrganizationTeacherApplicationsParams;
use super::errors::organization_teacher_application_list_error;
use super::teacher_application_dto::OrganizationTeacherApplicationsResponse;

pub(super) async fn get_organization_teacher_applications(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationTeacherApplicationListUseCase>>,
    query: web::Query<OrganizationTeacherApplicationsParams>,
) -> Result<web::Json<OrganizationTeacherApplicationsResponse>, ApiError> {
    let organization_id = path.into_inner();

    let list_query = OrganizationTeacherApplicationListQuery {
        actor_user_id: requester.user_id(),
        organization_id,
        status: query.status.clone(),
        search: query.search.clone(),
        limit: query.limit,
        offset: query.offset,
    };

    use_case
        .list_organization_teacher_applications(list_query)
        .await
        .map(OrganizationTeacherApplicationsResponse::from)
        .map(web::Json)
        .map_err(|error| organization_teacher_application_list_error(organization_id, error))
}
