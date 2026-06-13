use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentUseCase,
};

struct RouteOnlyOrganizationMemberRoleAssignmentUseCase;

pub fn organization_member_role_assignment_data(
) -> web::Data<Arc<dyn OrganizationMemberRoleAssignmentUseCase>> {
    web::Data::new(Arc::new(RouteOnlyOrganizationMemberRoleAssignmentUseCase)
        as Arc<dyn OrganizationMemberRoleAssignmentUseCase>)
}

impl OrganizationMemberRoleAssignmentUseCase for RouteOnlyOrganizationMemberRoleAssignmentUseCase {
    fn assign_organization_member_role(
        &self,
        command: OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<
        '_,
        Result<OrganizationMemberRoleAssignmentOutput, OrganizationMemberRoleAssignmentError>,
    > {
        ready(Ok(OrganizationMemberRoleAssignmentOutput {
            organization_id: command.organization_id,
            target_user_id: command.target_user_id,
            role_name: command.role_name,
        }))
        .boxed()
    }
}
