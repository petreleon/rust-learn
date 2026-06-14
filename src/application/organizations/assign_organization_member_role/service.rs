use futures::future::BoxFuture;

use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentOutput,
};

pub trait OrganizationMemberRoleAssignmentUseCase: Send + Sync {
    fn assign_organization_member_role(
        &self,
        command: OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<
        '_,
        Result<OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentError>,
    >;
}
