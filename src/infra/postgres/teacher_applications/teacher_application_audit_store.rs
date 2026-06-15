use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::teacher_applications::{
    list_application_audit::{TeacherApplicationAuditError, TeacherApplicationAuditStore},
    TeacherApplicationAuditEventOutput,
};
use crate::db::schema::teacher_application_audit_events;
use crate::infra::postgres::access_control::permission_checks;
use crate::models::teacher_application::TeacherApplicationAuditEvent;

pub struct PostgresTeacherApplicationAuditStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationAuditStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

pub async fn list_teacher_application_audit_events(
    conn: &mut AsyncPgConnection,
    application_id: i64,
) -> QueryResult<Vec<TeacherApplicationAuditEvent>> {
    teacher_application_audit_events::table
        .filter(teacher_application_audit_events::application_id.eq(application_id))
        .order(teacher_application_audit_events::created_at.asc())
        .load::<TeacherApplicationAuditEvent>(conn)
        .await
}

impl TeacherApplicationAuditStore for PostgresTeacherApplicationAuditStore<'_> {
    fn list_audit_events(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationAuditError>>
    {
        async move {
            list_teacher_application_audit_events(self.conn, application_id)
                .await
                .map(|events| events.into_iter().map(Into::into).collect())
                .map_err(map_error)
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresTeacherApplicationAuditStore<'_> {
    type Error = TeacherApplicationAuditError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationAuditError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_error)
        }
        .boxed()
    }
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationAuditError {
    TeacherApplicationAuditError::Database(error.to_string())
}
