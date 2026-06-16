use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::identity::list_platform_role_assignment_audit::{
    PlatformRoleAssignmentAuditError, PlatformRoleAssignmentAuditEventOutput,
};

pub trait PlatformRoleAssignmentAuditStore:
    AccessDecisionStore<Error = PlatformRoleAssignmentAuditError>
{
    fn target_user_exists(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRoleAssignmentAuditError>>;

    fn list_assignment_audit(
        &mut self,
        target_user_id: i32,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError>,
    >;
}
