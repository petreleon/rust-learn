use futures::future::BoxFuture;

use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
};

pub trait OrganizationMemberAuditUseCase: Send + Sync {
    fn list_member_audit(
        &self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>;
}
