use futures::future::BoxFuture;

use super::{AssignPlatformRoleCommand, AssignPlatformRoleError};

pub trait PlatformRoleAssignmentStore {
    fn assign_role(
        &mut self,
        command: AssignPlatformRoleCommand,
    ) -> BoxFuture<'_, Result<(), AssignPlatformRoleError>>;
}
