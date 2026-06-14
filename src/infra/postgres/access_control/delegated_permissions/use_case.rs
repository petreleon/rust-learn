use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::manage_delegated_permissions::{
    self, DelegatedPermissionError, DelegatedPermissionOutput, DelegatedPermissionUseCase,
    GrantDelegatedPermissionCommand, ListDelegatedPermissionsQuery,
    RevokeDelegatedPermissionCommand,
};
use crate::db::DbPool;
use crate::infra::postgres::access_control::delegated_permissions::store::PostgresDelegatedPermissionStore;

#[derive(Clone)]
pub struct PostgresDelegatedPermissionUseCase {
    pool: DbPool,
}

impl PostgresDelegatedPermissionUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl DelegatedPermissionUseCase for PostgresDelegatedPermissionUseCase {
    fn grant_delegated_permission(
        &self,
        command: GrantDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresDelegatedPermissionStore::new(&mut conn);
            manage_delegated_permissions::grant_delegated_permission(&mut store, command).await
        }
        .boxed()
    }

    fn list_delegated_permissions(
        &self,
        query: ListDelegatedPermissionsQuery,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresDelegatedPermissionStore::new(&mut conn);
            manage_delegated_permissions::list_delegated_permissions(&mut store, query).await
        }
        .boxed()
    }

    fn revoke_delegated_permission(
        &self,
        command: RevokeDelegatedPermissionCommand,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresDelegatedPermissionStore::new(&mut conn);
            manage_delegated_permissions::revoke_delegated_permission(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresDelegatedPermissionUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, DelegatedPermissionError> {
        self.pool
            .get()
            .await
            .map_err(|error| DelegatedPermissionError::Connection(error.to_string()))
    }
}
