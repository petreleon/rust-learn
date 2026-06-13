use futures::future::BoxFuture;

use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
};

pub trait OrganizationMemberRoleAssignmentStore {
    fn can_assign_role(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRoleAssignmentError>>;

    fn actor_min_level(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>>;

    fn target_min_level(
        &mut self,
        target_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>>;

    fn role_id_by_name(
        &mut self,
        role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>>;

    fn role_hierarchy_level(
        &mut self,
        role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>>;

    fn assign_role(
        &mut self,
        target_user_id: i32,
        organization_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>>;

    fn record_role_assignment(
        &mut self,
        command: &OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>>;
}
