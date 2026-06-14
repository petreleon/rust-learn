use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::list_roles::{self, RoleCatalogUseCase};
use crate::application::access_control::role_catalog::{RoleCatalogEntry, RoleCatalogError};
use crate::db::DbPool;
use crate::infra::postgres::access_control::role_catalog_store::PostgresRoleCatalogStore;

#[derive(Clone)]
pub struct PostgresRoleCatalogUseCase {
    pool: DbPool,
}

impl PostgresRoleCatalogUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl RoleCatalogUseCase for PostgresRoleCatalogUseCase {
    fn list_platform_roles(
        &self,
    ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRoleCatalogStore::new(&mut conn);
            list_roles::list_platform_roles(&mut store).await
        }
        .boxed()
    }

    fn list_organization_roles(
        &self,
    ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRoleCatalogStore::new(&mut conn);
            list_roles::list_organization_roles(&mut store).await
        }
        .boxed()
    }

    fn list_course_roles(&self) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresRoleCatalogStore::new(&mut conn);
            list_roles::list_course_roles(&mut store).await
        }
        .boxed()
    }
}

impl PostgresRoleCatalogUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        RoleCatalogError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| RoleCatalogError::Connection(error.to_string()))
    }
}
