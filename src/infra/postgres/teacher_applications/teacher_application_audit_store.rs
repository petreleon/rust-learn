use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    list_application_audit::{TeacherApplicationAuditError, TeacherApplicationAuditStore},
    TeacherApplicationAuditEventOutput,
};
use crate::config::constants::permissions::Permissions;
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
    fn can_review_teacher_applications(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationAuditError>> {
        async move {
            let permission = Permissions::REVIEW_TEACHER_APPLICATIONS.to_string();
            permission_checks::has_platform_permission(self.conn, actor_user_id, &permission)
                .await
                .map_err(map_error)
        }
        .boxed()
    }

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

fn map_error(error: diesel::result::Error) -> TeacherApplicationAuditError {
    TeacherApplicationAuditError::Database(error.to_string())
}
