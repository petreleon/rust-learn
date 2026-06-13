use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalStore,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{organization_member_audit_events, user_role_organization};
use crate::infra::postgres::organizations::organization_member_removal_mappers::map_member_removal_error;
use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;
use crate::repositories::organization_repository::user_permission_organization_request;

pub struct PostgresOrganizationMemberRemovalStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberRemovalStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberRemovalStore for PostgresOrganizationMemberRemovalStore<'_> {
    fn can_remove_member(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRemovalError>> {
        async move {
            user_permission_organization_request(
                self.conn,
                actor_user_id,
                organization_id,
                &Permissions::MANAGE_ORG_MEMBERS.to_string(),
            )
            .await
            .map_err(map_member_removal_error)
        }
        .boxed()
    }

    fn remove_member(
        &mut self,
        command: OrganizationMemberRemovalCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRemovalError>> {
        async move { remove_member(self.conn, command).await }.boxed()
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
            event_type: "member_removed".to_string(),
            role_name: None,
            reason: None,
        })
        .execute(conn)
        .await
}
