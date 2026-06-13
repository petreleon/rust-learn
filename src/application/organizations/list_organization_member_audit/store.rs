use futures::future::BoxFuture;

use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
};

pub trait OrganizationMemberAuditStore {
    fn can_view_member_audit(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberAuditError>>;

    fn list_member_audit_events(
        &mut self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>;
}
