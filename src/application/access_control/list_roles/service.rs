use futures::future::BoxFuture;

use crate::application::access_control::role_catalog::{RoleCatalogEntry, RoleCatalogError};

pub trait RoleCatalogUseCase: Send + Sync {
    fn list_platform_roles(&self)
        -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>>;

    fn list_organization_roles(
        &self,
    ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>>;

    fn list_course_roles(&self) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>>;
}
