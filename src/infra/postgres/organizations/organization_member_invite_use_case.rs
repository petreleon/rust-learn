use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::invite_organization_member::{
    self, OrganizationMemberInviteCommand, OrganizationMemberInviteError,
    OrganizationMemberInviteOutput, OrganizationMemberInviteUseCase,
};
use crate::infra::postgres::organizations::organization_member_invite_store::PostgresOrganizationMemberInviteStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresOrganizationMemberInviteUseCase {
    pool: DbPool,
}

impl PostgresOrganizationMemberInviteUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationMemberInviteUseCase for PostgresOrganizationMemberInviteUseCase {
    fn invite_organization_member(
        &self,
        command: OrganizationMemberInviteCommand,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteOutput, OrganizationMemberInviteError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationMemberInviteStore::new(&mut conn);
            invite_organization_member::invite_organization_member(&mut store, command).await
        }
        .boxed()
    }
}

impl PostgresOrganizationMemberInviteUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, OrganizationMemberInviteError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationMemberInviteError::Connection(error.to_string()))
    }
}
