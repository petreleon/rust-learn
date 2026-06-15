use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListQuery, OrganizationCourseListUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::course_dto::OrganizationCourseListResponse;
use super::dto::OrganizationCourseListParams;
use super::errors::organization_course_list_error;

pub(super) async fn get_organization_courses(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn OrganizationCourseListUseCase>>,
    query: web::Query<OrganizationCourseListParams>,
) -> Result<web::Json<OrganizationCourseListResponse>, ApiError> {
    let organization_id = path.into_inner();

    let list_query = OrganizationCourseListQuery::new(
        requester.user_id(),
        organization_id,
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    use_case
        .list_organization_courses(list_query)
        .await
        .map(OrganizationCourseListResponse::from)
        .map(web::Json)
        .map_err(|error| organization_course_list_error(organization_id, error))
}
