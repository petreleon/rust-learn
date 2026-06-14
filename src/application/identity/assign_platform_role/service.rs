use futures::future::BoxFuture;

use super::{AssignPlatformRoleCommand, AssignPlatformRoleError, AssignPlatformRoleOutcome};

pub trait PlatformRoleAssignmentUseCase: Send + Sync {
    fn assign_platform_role(
        &self,
        command: AssignPlatformRoleCommand,
    ) -> BoxFuture<'_, Result<AssignPlatformRoleOutcome, AssignPlatformRoleError>>;
}
