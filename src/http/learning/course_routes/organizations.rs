use std::sync::Arc;

use actix_web::web;

use crate::application::learning::list_course_organizations::CourseOrganizationsUseCase;
use crate::http::errors::ApiError;
use crate::http::learning::dto::CourseOrganizationResponse;

use super::errors::course_organizations_error;

pub(super) async fn get_course_organizations(
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn CourseOrganizationsUseCase>>,
) -> Result<web::Json<Vec<CourseOrganizationResponse>>, ApiError> {
    let course_id = path.into_inner();

    use_case
        .list_course_organizations(course_id)
        .await
        .map(organization_responses)
        .map(web::Json)
        .map_err(|error| course_organizations_error(course_id, error))
}

fn organization_responses(
    organizations: Vec<
        crate::application::learning::list_course_organizations::CourseOrganizationOutput,
    >,
) -> Vec<CourseOrganizationResponse> {
    organizations
        .into_iter()
        .map(CourseOrganizationResponse::from)
        .collect()
}
