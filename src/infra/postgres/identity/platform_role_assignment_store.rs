use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::assign_platform_role::{
    AssignPlatformRoleCommand, AssignPlatformRoleError, PlatformRoleAssignmentStore,
};
use crate::repositories::platform_repository::assign_role_to_user_with_hierarchy;

pub struct PostgresPlatformRoleAssignmentStore<'conn> {
    conn: &'conn mut diesel_async::AsyncPgConnection,
}

impl<'conn> PostgresPlatformRoleAssignmentStore<'conn> {
    pub fn new(conn: &'conn mut diesel_async::AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformRoleAssignmentStore for PostgresPlatformRoleAssignmentStore<'_> {
    fn assign_role(
        &mut self,
        command: AssignPlatformRoleCommand,
    ) -> BoxFuture<'_, Result<(), AssignPlatformRoleError>> {
        async move {
            assign_role_to_user_with_hierarchy(
                self.conn,
                command.requester_user_id,
                command.target_user_id,
                &command.role_name,
            )
            .await
            .map(|_| ())
            .map_err(map_role_assignment_error)
        }
        .boxed()
    }
}

fn map_role_assignment_error(error: diesel::result::Error) -> AssignPlatformRoleError {
    match error {
        diesel::result::Error::RollbackTransaction => AssignPlatformRoleError::HierarchyViolation,
        diesel::result::Error::NotFound => AssignPlatformRoleError::RoleNotFound,
        other => AssignPlatformRoleError::Database(other.to_string()),
    }
}
