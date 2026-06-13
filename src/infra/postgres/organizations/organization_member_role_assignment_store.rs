use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentStore,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    organization_member_audit_events, organization_roles, role_organization_hierarchy,
    user_role_organization,
};
use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;
use crate::repositories::organization_repository::user_permission_organization_request;

pub struct PostgresOrganizationMemberRoleAssignmentStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberRoleAssignmentStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberRoleAssignmentStore for PostgresOrganizationMemberRoleAssignmentStore<'_> {
    fn can_assign_role(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationMemberRoleAssignmentError>> {
        async move {
            user_permission_organization_request(
                self.conn,
                actor_user_id,
                organization_id,
                &Permissions::ASSIGN_ROLES_TO_ORG_USERS.to_string(),
            )
            .await
            .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn actor_min_level(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        async move {
            organization_min_level(self.conn, actor_user_id, organization_id)
                .await
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn target_min_level(
        &mut self,
        target_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        async move {
            organization_min_level(self.conn, target_user_id, organization_id)
                .await
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn role_id_by_name(
        &mut self,
        role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        let role_name = role_name.to_string();
        async move {
            organization_roles::table
                .filter(organization_roles::name.eq(role_name))
                .select(organization_roles::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn role_hierarchy_level(
        &mut self,
        role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, OrganizationMemberRoleAssignmentError>> {
        async move {
            role_organization_hierarchy::table
                .filter(role_organization_hierarchy::organization_role_id.eq(role_id))
                .select(role_organization_hierarchy::hierarchy_level)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn assign_role(
        &mut self,
        target_user_id: i32,
        organization_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>> {
        async move {
            diesel::insert_into(user_role_organization::table)
                .values((
                    user_role_organization::user_id.eq(target_user_id),
                    user_role_organization::organization_id.eq(organization_id),
                    user_role_organization::organization_role_id.eq(role_id),
                ))
                .execute(self.conn)
                .await
                .map(|_| ())
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn record_role_assignment(
        &mut self,
        command: &OrganizationMemberRoleAssignmentCommand,
    ) -> BoxFuture<'_, Result<(), OrganizationMemberRoleAssignmentError>> {
        let command = command.clone();
        async move {
            diesel::insert_into(organization_member_audit_events::table)
                .values(NewOrganizationMemberAuditEvent {
                    organization_id: command.organization_id,
                    actor_user_id: Some(command.actor_user_id),
                    target_user_id: command.target_user_id,
                    event_type: "role_assigned".to_string(),
                    role_name: Some(command.role_name),
                    reason: None,
                })
                .execute(self.conn)
                .await
                .map(|_| ())
                .map_err(map_assignment_error)
        }
        .boxed()
    }
}

async fn organization_min_level(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
) -> diesel::QueryResult<Option<i32>> {
    role_organization_hierarchy::table
        .inner_join(
            user_role_organization::table.on(role_organization_hierarchy::organization_role_id
                .eq(user_role_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(user_id))
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(diesel::dsl::min(
            role_organization_hierarchy::hierarchy_level,
        ))
        .first::<Option<i32>>(conn)
        .await
}

fn map_assignment_error(error: diesel::result::Error) -> OrganizationMemberRoleAssignmentError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberRoleAssignmentError::NotFound,
        other => OrganizationMemberRoleAssignmentError::Database(other.to_string()),
    }
}
