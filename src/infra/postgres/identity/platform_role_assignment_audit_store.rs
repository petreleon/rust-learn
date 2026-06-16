use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::identity::list_platform_role_assignment_audit::{
    PlatformRoleAssignmentAuditError, PlatformRoleAssignmentAuditEventOutput,
    PlatformRoleAssignmentAuditStore,
};
use crate::domain::access_control::role_assignment_audit::PlatformRoleAssignmentAuditEventType;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::platform_role_assignment_audit_event::PlatformRoleAssignmentAuditEvent;
use crate::infra::postgres::schema::{platform_role_assignment_audit_events, users};

pub struct PostgresPlatformRoleAssignmentAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresPlatformRoleAssignmentAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformRoleAssignmentAuditStore for PostgresPlatformRoleAssignmentAuditStore<'_> {
    fn target_user_exists(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, PlatformRoleAssignmentAuditError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map(|found| found.is_some())
                .map_err(map_audit_error)
        }
        .boxed()
    }

    fn list_assignment_audit(
        &mut self,
        target_user_id: i32,
    ) -> BoxFuture<
        '_,
        Result<Vec<PlatformRoleAssignmentAuditEventOutput>, PlatformRoleAssignmentAuditError>,
    > {
        async move {
            let events = platform_role_assignment_audit_events::table
                .filter(platform_role_assignment_audit_events::target_user_id.eq(target_user_id))
                .order(platform_role_assignment_audit_events::created_at.desc())
                .then_order_by(platform_role_assignment_audit_events::id.desc())
                .limit(50)
                .load::<PlatformRoleAssignmentAuditEvent>(self.conn)
                .await
                .map_err(map_audit_error)?;
            events.into_iter().map(audit_output).collect()
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresPlatformRoleAssignmentAuditStore<'_> {
    type Error = PlatformRoleAssignmentAuditError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, PlatformRoleAssignmentAuditError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_audit_error)
        }
        .boxed()
    }
}

fn audit_output(
    event: PlatformRoleAssignmentAuditEvent,
) -> Result<PlatformRoleAssignmentAuditEventOutput, PlatformRoleAssignmentAuditError> {
    let event_type = PlatformRoleAssignmentAuditEventType::parse(&event.event_type)
        .map_err(|error| PlatformRoleAssignmentAuditError::Database(error.to_string()))?;
    Ok(PlatformRoleAssignmentAuditEventOutput {
        actor_user_id: event.actor_user_id,
        created_at: event.created_at,
        event_type,
        id: event.id,
        platform_role_id: event.platform_role_id,
        role_name: event.role_name,
        target_user_id: event.target_user_id,
    })
}

fn map_audit_error(error: diesel::result::Error) -> PlatformRoleAssignmentAuditError {
    match error {
        diesel::result::Error::NotFound => PlatformRoleAssignmentAuditError::UserNotFound,
        other => PlatformRoleAssignmentAuditError::Database(other.to_string()),
    }
}
