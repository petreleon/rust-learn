use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::assign_organization_member_role::{
    self, OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_member_role_assignment_store::PostgresOrganizationMemberRoleAssignmentStore;

#[derive(Clone)]
pub struct PostgresOrganizationMemberRoleAssignmentUseCase {
    pool: DbPool,
}

impl PostgresOrganizationMemberRoleAssignmentUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl OrganizationMemberRoleAssignmentUseCase for PostgresOrganizationMemberRoleAssignmentUseCase {
    fn assign_organization_member_role(
        &self,
        command: OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<
        '_,
        Result<OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresOrganizationMemberRoleAssignmentStore::new(&mut conn);
            assign_organization_member_role::assign_organization_member_role(&mut store, command)
                .await
        }
        .boxed()
    }
}

impl PostgresOrganizationMemberRoleAssignmentUseCase {
    async fn connection(
        &self,
    ) -> Result<Object<AsyncPgConnection>, OrganizationMemberRoleAssignmentError> {
        self.pool
            .get()
            .await
            .map_err(|error| OrganizationMemberRoleAssignmentError::Connection(error.to_string()))
    }
}
