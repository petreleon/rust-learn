use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteError, OrganizationMemberInviteStore, OrganizationMemberInviteTarget,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::organization_member_audit_events;
use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;
use crate::models::user::User;
use crate::repositories::organization_repository::{
    assign_role_to_user_in_organization, user_permission_organization_request,
};

pub struct PostgresOrganizationMemberInviteStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberInviteStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberInviteStore for PostgresOrganizationMemberInviteStore<'_> {
    fn can_invite_member(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberInviteError>> {
        async move {
            user_permission_organization_request(
                self.conn,
                actor_user_id,
                organization_id,
                &Permissions::INVITE_USER_TO_ORGANIZATION.to_string(),
            )
            .await
            .map_err(map_member_invite_error)
        }
        .boxed()
    }

    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<OrganizationMemberInviteTarget, OrganizationMemberInviteError>> {
        async move {
            let user = User::find_by_email(&email, self.conn)
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
            assign_role_to_user_in_organization(
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
            event_type: "role_assigned".to_string(),
            role_name: Some(role_name.to_string()),
            reason: None,
        })
        .execute(conn)
        .await
}
