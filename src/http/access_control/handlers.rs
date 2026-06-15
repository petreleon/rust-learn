use std::sync::Arc;

use actix_web::web;

use crate::application::access_control::list_roles::RoleCatalogUseCase;
use crate::application::access_control::role_catalog::RoleCatalogEntry;
use crate::http::access_control::dto::RoleResponse;
use crate::http::access_control::errors::role_catalog_error;
use crate::http::errors::ApiError;

pub async fn list_platform_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> Result<web::Json<Vec<RoleResponse>>, ApiError> {
    role_catalog
        .list_platform_roles()
        .await
        .map(role_responses)
        .map(web::Json)
        .map_err(|error| role_catalog_error("platform", error))
}

pub async fn list_organization_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> Result<web::Json<Vec<RoleResponse>>, ApiError> {
    role_catalog
        .list_organization_roles()
        .await
        .map(role_responses)
        .map(web::Json)
        .map_err(|error| role_catalog_error("organization", error))
}

pub async fn list_course_roles(
    role_catalog: web::Data<Arc<dyn RoleCatalogUseCase>>,
) -> Result<web::Json<Vec<RoleResponse>>, ApiError> {
    role_catalog
        .list_course_roles()
        .await
        .map(role_responses)
        .map(web::Json)
        .map_err(|error| role_catalog_error("course", error))
}

fn role_responses(roles: Vec<RoleCatalogEntry>) -> Vec<RoleResponse> {
    roles.into_iter().map(RoleResponse::from).collect()
}
