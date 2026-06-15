use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditStore,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::organization_member_audit_events;
use crate::infra::postgres::organizations::organization_member_audit_mappers::{
    map_member_audit_error, organization_member_audit_output_from_model,
};
use crate::infra::postgres::organizations::organization_permission_checks::can_organization_permission;
use crate::models::organization_member_audit_event::OrganizationMemberAuditEvent;

pub struct PostgresOrganizationMemberAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberAuditStore for PostgresOrganizationMemberAuditStore<'_> {
    fn can_view_member_audit(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberAuditError>> {
        async move {
            can_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                Permissions::VIEW_ORGANIZATION,
            )
            .await
            .map_err(map_member_audit_error)
        }
        .boxed()
    }

    fn list_member_audit_events(
        &mut self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>
    {
        async move { list_member_audit_events(self.conn, query).await }.boxed()
    }
}

async fn list_member_audit_events(
    conn: &mut AsyncPgConnection,
    query: OrganizationMemberAuditQuery,
) -> Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError> {
    organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(query.organization_id))
        .filter(organization_member_audit_events::target_user_id.eq(query.target_user_id))
        .order(organization_member_audit_events::created_at.desc())
        .load::<OrganizationMemberAuditEvent>(conn)
        .await
        .map(|events| {
            events
                .into_iter()
                .map(organization_member_audit_output_from_model)
                .collect()
        })
        .map_err(map_member_audit_error)
}
