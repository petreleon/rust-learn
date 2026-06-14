use futures::future::BoxFuture;

use crate::application::access_control::role_catalog::{
    RoleCatalogEntry, RoleCatalogError, RoleCatalogScope,
};

pub trait RoleCatalogStore {
    fn list_roles(
        &mut self,
        scope: RoleCatalogScope,
    ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>>;
}
