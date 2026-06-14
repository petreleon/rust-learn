use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_members::{
    self, OrganizationMemberListError, OrganizationMemberListOutput, OrganizationMemberListQuery,
    OrganizationMemberListUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_member_list_store::PostgresOrganizationMemberListStore;

#[derive(Clone)]
pub struct PostgresOrganizationMemberListUseCase {
    pool: DbPool,
}

impl PostgresOrganizationMemberListUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationMemberListUseCase for PostgresOrganizationMemberListUseCase {
    fn list_organization_members(
        &self,
        query: OrganizationMemberListQuery,
    ) -> BoxFuture<'_, Result<OrganizationMemberListOutput, OrganizationMemberListError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationMemberListStore::new(&mut conn);
            list_organization_members::list_organization_members(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresOrganizationMemberListUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationMemberListError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationMemberListError::Connection(error.to_string()))
    }
}
