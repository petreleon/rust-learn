use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteError, OrganizationMemberInviteStore, OrganizationMemberInviteTarget,
};
use crate::domain::organizations::member_audit::OrganizationMemberAuditEventType;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::identity::accounts::find_user_by_email;
use crate::infra::postgres::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;
use crate::infra::postgres::organizations::organization_role_assignments::assign_role_with_hierarchy;
use crate::infra::postgres::schema::organization_member_audit_events;

pub struct PostgresOrganizationMemberInviteStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberInviteStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberInviteStore for PostgresOrganizationMemberInviteStore<'_> {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteTarget, OrganizationMemberInviteError>> {
        async move {
            let user = find_user_by_email(self.conn, &email)
                .await
                .map_err(map_user_lookup_error)?;
            Ok(OrganizationMemberInviteTarget {
                user_id: user.id,
                name: user.name,
                email: user.email,
            })
        }
        .boxed()
    }

    fn assign_member_role(
        &mut self,
        actor_user_id: i32,
        target_user_id: i32,
        organization_id: i32,
        role_name: String,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberInviteError>> {
        async move {
            assign_role_with_hierarchy(
                self.conn,
                actor_user_id,
                target_user_id,
                organization_id,
                &role_name,
            )
            .await
            .map_err(map_role_assignment_error)?;

            log_role_assigned_event(
                self.conn,
                actor_user_id,
                target_user_id,
                organization_id,
                &role_name,
            )
            .await
            .ok();

            Ok(())
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresOrganizationMemberInviteStore<'_> {
    type Error = OrganizationMemberInviteError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberInviteError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_member_invite_error)
        }
        .boxed()
    }
}

fn map_user_lookup_error(error: diesel::result::Error) -> OrganizationMemberInviteError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberInviteError::UserNotFound,
        other => OrganizationMemberInviteError::Database(other.to_string()),
    }
}

fn map_role_assignment_error(error: diesel::result::Error) -> OrganizationMemberInviteError {
    match error {
        diesel::result::Error::RollbackTransaction => {
            OrganizationMemberInviteError::HierarchyDenied
        }
        diesel::result::Error::NotFound => OrganizationMemberInviteError::RoleOrUserNotFound,
        other => OrganizationMemberInviteError::Database(other.to_string()),
    }
}

fn map_member_invite_error(error: diesel::result::Error) -> OrganizationMemberInviteError {
    OrganizationMemberInviteError::Database(error.to_string())
}

async fn log_role_assigned_event(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    target_user_id: i32,
    organization_id: i32,
    role_name: &str,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(organization_member_audit_events::table)
        .values(NewOrganizationMemberAuditEvent {
            organization_id,
            actor_user_id: Some(actor_user_id),
            target_user_id,
            event_type: OrganizationMemberAuditEventType::RoleAssigned
                .as_str()
                .to_string(),
            role_name: Some(role_name.to_string()),
            reason: None,
        })
        .execute(conn)
        .await
}
