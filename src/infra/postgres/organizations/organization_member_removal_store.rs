use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalStore,
};
use crate::db::schema::{organization_member_audit_events, user_role_organization};
use crate::domain::organizations::member_audit::OrganizationMemberAuditEventType;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::organizations::organization_member_removal_mappers::map_member_removal_error;
use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;

pub struct PostgresOrganizationMemberRemovalStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberRemovalStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberRemovalStore for PostgresOrganizationMemberRemovalStore<'_> {
    fn remove_member(
        &mut self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>> {
        async move { remove_member(self.conn, command).await }.boxed()
    }
}

impl AccessDecisionStore for PostgresOrganizationMemberRemovalStore<'_> {
    type Error = OrganizationMemberRemovalError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRemovalError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_member_removal_error)
        }
        .boxed()
    }
}

async fn remove_member(
    conn: &mut AsyncPgConnection,
    command: OrganizationMemberRemovalCommand,
) -> Result<(), OrganizationMemberRemovalError> {
    let deleted = diesel::delete(
        user_role_organization::table
            .filter(user_role_organization::organization_id.eq(command.organization_id))
            .filter(user_role_organization::user_id.eq(command.target_user_id)),
    )
    .execute(conn)
    .await
    .map_err(map_member_removal_error)?;

    if deleted == 0 {
        return Err(OrganizationMemberRemovalError::NotFound);
    }

    log_member_removed_event(conn, command).await.ok();
    Ok(())
}

async fn log_member_removed_event(
    conn: &mut AsyncPgConnection,
    command: OrganizationMemberRemovalCommand,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(organization_member_audit_events::table)
        .values(NewOrganizationMemberAuditEvent {
            organization_id: command.organization_id,
            actor_user_id: None,
            target_user_id: command.target_user_id,
            event_type: OrganizationMemberAuditEventType::MemberRemoved
                .as_str()
                .to_string(),
            role_name: None,
            reason: None,
        })
        .execute(conn)
        .await
}
