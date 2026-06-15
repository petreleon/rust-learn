use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditStore,
};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::organization_member_audit_event::OrganizationMemberAuditEvent;
use crate::infra::postgres::organizations::organization_member_audit_mappers::{
    map_member_audit_error, organization_member_audit_output_from_model,
};
use crate::infra::postgres::schema::organization_member_audit_events;

pub struct PostgresOrganizationMemberAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberAuditStore for PostgresOrganizationMemberAuditStore<'_> {
    fn list_member_audit_events(
        &mut self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>
    {
        async move { list_member_audit_events(self.conn, query).await }.boxed()
    }
}

impl AccessDecisionStore for PostgresOrganizationMemberAuditStore<'_> {
    type Error = OrganizationMemberAuditError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberAuditError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_member_audit_error)
        }
        .boxed()
    }
}

async fn list_member_audit_events(
    conn: &mut AsyncPgConnection,
    query: OrganizationMemberAuditQuery,
) -> Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError> {
    let events = organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(query.organization_id))
        .filter(organization_member_audit_events::target_user_id.eq(query.target_user_id))
        .order(organization_member_audit_events::created_at.desc())
        .load::<OrganizationMemberAuditEvent>(conn)
        .await
        .map_err(map_member_audit_error)?;
    events
        .into_iter()
        .map(organization_member_audit_output_from_model)
        .collect()
}
