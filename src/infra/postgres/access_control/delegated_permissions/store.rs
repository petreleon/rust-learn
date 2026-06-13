use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionCreate, DelegatedPermissionError, DelegatedPermissionFilter,
    DelegatedPermissionOutput, DelegatedPermissionStore,
};
use crate::infra::postgres::access_control::delegated_permissions::{read_queries, write_queries};

pub struct PostgresDelegatedPermissionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresDelegatedPermissionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl DelegatedPermissionStore for PostgresDelegatedPermissionStore<'_> {
    fn can_delegate_reward_permissions(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { read_queries::can_delegate_reward_permissions(self.conn, user_id).await }
            .boxed()
    }

    fn organization_exists(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { read_queries::organization_exists(self.conn, organization_id).await }.boxed()
    }

    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, DelegatedPermissionError>> {
        async move { read_queries::course_exists(self.conn, course_id).await }.boxed()
    }

    fn find_active_delegated_permission(
        &mut self,
        grantee_user_id: i32,
        permission: String,
        scope_type: String,
        organization_id: Option<i32>,
        course_id: Option<i32>,
    ) -> BoxFuture<'_, Result<Option<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        async move {
            read_queries::find_active_delegated_permission(
                self.conn,
                grantee_user_id,
                &permission,
                &scope_type,
                organization_id,
                course_id,
            )
            .await
        }
        .boxed()
    }

    fn create_delegated_permission(
        &mut self,
        delegation: DelegatedPermissionCreate,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move { write_queries::create_delegated_permission(self.conn, delegation).await }
            .boxed()
    }

    fn list_delegated_permissions(
        &mut self,
        filter: DelegatedPermissionFilter,
    ) -> BoxFuture<'_, Result<Vec<DelegatedPermissionOutput>, DelegatedPermissionError>> {
        async move { read_queries::list_delegated_permissions(self.conn, filter).await }.boxed()
    }

    fn revoke_delegated_permission(
        &mut self,
        delegation_id: i64,
        revoked_by_user_id: i32,
        revoke_reason: Option<String>,
    ) -> BoxFuture<'_, Result<DelegatedPermissionOutput, DelegatedPermissionError>> {
        async move {
            write_queries::revoke_delegated_permission(
                self.conn,
                delegation_id,
                revoked_by_user_id,
                revoke_reason,
            )
            .await
        }
        .boxed()
    }
}
