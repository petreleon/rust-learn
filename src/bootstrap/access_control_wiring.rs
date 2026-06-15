use std::sync::Arc;

use actix_web::web;

use crate::application::access_control::check_permission::AccessDecisionService;
use crate::application::access_control::compare_hierarchy::HierarchyCheckService;
use crate::application::access_control::list_roles::RoleCatalogUseCase;
use crate::application::access_control::manage_delegated_permissions::DelegatedPermissionUseCase;
use crate::db::DbPool;
use crate::infra::postgres::access_control::delegated_permissions::use_case::PostgresDelegatedPermissionUseCase;
use crate::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;

#[derive(Clone)]
pub struct AccessControlUseCases {
    pub access_decision: AccessDecisionService,
    pub delegated_permissions: Arc<dyn DelegatedPermissionUseCase>,
    pub hierarchy_check: HierarchyCheckService,
    pub role_catalog: Arc<dyn RoleCatalogUseCase>,
}

pub fn build_access_control_use_cases(pool: &DbPool) -> AccessControlUseCases {
    AccessControlUseCases {
        access_decision: Arc::new(pool.clone()),
        delegated_permissions: Arc::new(PostgresDelegatedPermissionUseCase::new(pool.clone())),
        hierarchy_check: Arc::new(pool.clone()),
        role_catalog: Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())),
    }
}

pub fn configure_access_control_app_data(
    cfg: &mut web::ServiceConfig,
    use_cases: &AccessControlUseCases,
) {
    cfg.app_data(web::Data::new(use_cases.delegated_permissions.clone()))
        .app_data(web::Data::new(use_cases.access_decision.clone()))
        .app_data(web::Data::new(use_cases.hierarchy_check.clone()))
        .app_data(web::Data::new(use_cases.role_catalog.clone()));
}

pub(crate) fn configure_access_control_check_app_data(cfg: &mut web::ServiceConfig, pool: &DbPool) {
    let hierarchy_check: HierarchyCheckService = Arc::new(pool.clone());
    let access_decision: AccessDecisionService = Arc::new(pool.clone());

    cfg.app_data(web::Data::new(hierarchy_check))
        .app_data(web::Data::new(access_decision));
}
