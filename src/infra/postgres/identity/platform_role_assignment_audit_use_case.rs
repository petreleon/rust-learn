use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::list_platform_role_assignment_audit::{
    self, PlatformRoleAssignmentAuditError, PlatformRoleAssignmentAuditEventOutput,
    PlatformRoleAssignmentAuditQuery, PlatformRoleAssignmentAuditUseCase,
};
use crate::infra::postgres::identity::platform_role_assignment_audit_store::PostgresPlatformRoleAssignmentAuditStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresPlatformRoleAssignmentAuditUseCase {
    pool: DbPool,
}

impl PostgresPlatformRoleAssignmentAuditUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PlatformRoleAssignmentAuditUseCase for PostgresPlatformRoleAssignmentAuditUseCase {
    fn list_platform_role_assignment_audit(
        &self,
        query: PlatformRoleAssignmentAuditQuery,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError>,
    > {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresPlatformRoleAssignmentAuditStore::new(&mut conn);
            list_platform_role_assignment_audit::list_platform_role_assignment_audit(
                &mut store, query,
            )
            .await
        }
        .boxed()
    }
}

impl PostgresPlatformRoleAssignmentAuditUseCase {
    async fn connection(
        &self,
    ) -> Result<
        diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>,
        PlatformRoleAssignmentAuditError,
    > {
        self.pool
            .get()
            .await
            .map_err(|error| PlatformRoleAssignmentAuditError::Connection(error.to_string()))
    }
}
