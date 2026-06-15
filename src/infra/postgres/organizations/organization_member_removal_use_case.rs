use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::remove_organization_member::{
    self, OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalUseCase,
};
use crate::infra::postgres::organizations::organization_member_removal_store::PostgresOrganizationMemberRemovalStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresOrganizationMemberRemovalUseCase {
    pool: DbPool,
}

impl PostgresOrganizationMemberRemovalUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationMemberRemovalUseCase for PostgresOrganizationMemberRemovalUseCase {
    fn remove_organization_member(
        &self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationMemberRemovalStore::new(&mut conn);
            remove_organization_member::remove_organization_member(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresOrganizationMemberRemovalUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, OrganizationMemberRemovalError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationMemberRemovalError::Connection(error.to_string()))
    }
}
