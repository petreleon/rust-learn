use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_member_audit::{
    self, OrganizationMemberAuditError, OrganizationMemberAuditEventOutput,
    OrganizationMemberAuditQuery, OrganizationMemberAuditUseCase,
};
use crate::infra::postgres::organizations::organization_member_audit_store::PostgresOrganizationMemberAuditStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresOrganizationMemberAuditUseCase {
    pool: DbPool,
}

impl PostgresOrganizationMemberAuditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationMemberAuditUseCase for PostgresOrganizationMemberAuditUseCase {
    fn list_member_audit(
        &self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>
    {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationMemberAuditStore::new(&mut conn);
            list_organization_member_audit::list_organization_member_audit(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresOrganizationMemberAuditUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationMemberAuditError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationMemberAuditError::Connection(error.to_string()))
    }
}
