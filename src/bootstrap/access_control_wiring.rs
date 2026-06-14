use std::sync::Arc;

use actix_web::web;

use crate::application::access_control::list_roles::RoleCatalogUseCase;
use crate::application::access_control::manage_delegated_permissions::DelegatedPermissionUseCase;
use crate::db::DbPool;
use crate::infra::postgres::access_control::delegated_permissions::use_case::PostgresDelegatedPermissionUseCase;
use crate::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;

#[derive(Clone)]
pub struct AccessControlUseCases {
    pub delegated_permissions: Arc<dyn DelegatedPermissionUseCase>,
    pub role_catalog: Arc<dyn RoleCatalogUseCase>,
}

pub fn build_access_control_use_cases(pool: &DbPool) -> AccessControlUseCases {
    AccessControlUseCases {
        delegated_permissions: Arc::new(PostgresDelegatedPermissionUseCase::new(pool.clone())),
        role_catalog: Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())),
    }
}

pub fn configure_access_control_app_data(
    cfg: &mut web::ServiceConfig,
    use_cases: &AccessControlUseCases,
) {
    cfg.app_data(web::Data::new(use_cases.delegated_permissions.clone()))
        .app_data(web::Data::new(use_cases.role_catalog.clone()));
}
