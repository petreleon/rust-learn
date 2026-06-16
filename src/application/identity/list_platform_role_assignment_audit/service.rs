use futures::future::BoxFuture;

use crate::application::identity::list_platform_role_assignment_audit::{
    PlatformRoleAssignmentAuditError, PlatformRoleAssignmentAuditEventOutput,
    PlatformRoleAssignmentAuditQuery,
};

pub trait PlatformRoleAssignmentAuditUseCase: Send + Sync {
    fn list_platform_role_assignment_audit(
        &self,
        query: PlatformRoleAssignmentAuditQuery,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError>,
    >;
}
