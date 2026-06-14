use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::manage_organizations::{
    self, OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementUseCase,
    OrganizationOutput, OrganizationUpdateCommand,
};
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_management_store::PostgresOrganizationManagementStore;

#[derive(Clone)]
pub struct PostgresOrganizationManagementUseCase {
    pool: DbPool,
}

impl PostgresOrganizationManagementUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationManagementUseCase for PostgresOrganizationManagementUseCase {
    fn list_organizations(
        &self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationManagementStore::new(&mut conn);
            manage_organizations::list_organizations(&mut store).await
        }
        .boxed()
    }

    fn get_organization(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationManagementStore::new(&mut conn);
            manage_organizations::get_organization(&mut store, organization_id).await
        }
        .boxed()
    }

    fn create_organization(
        &self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationManagementStore::new(&mut conn);
            manage_organizations::create_organization(&mut store, command).await
        }
        .boxed()
    }

    fn update_organization(
        &self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationManagementStore::new(&mut conn);
            manage_organizations::update_organization(&mut store, command).await
        }
        .boxed()
    }

    fn delete_organization(
        &self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationManagementError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationManagementStore::new(&mut conn);
            manage_organizations::delete_organization(&mut store, organization_id).await
        }
        .boxed()
    }
}

impl PostgresOrganizationManagementUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationManagementError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationManagementError::Connection(error.to_string()))
    }
}
